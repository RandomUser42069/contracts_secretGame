use cosmwasm_std::{Api, Binary, CosmosMsg, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage, Uint128, WasmMsg, from_binary, to_binary};

use rand::prelude::*;
use secret_toolkit::utils::{HandleCallback, Query};
use sha2::{Sha256, Digest}; 
use rand_chacha::ChaChaRng;
 
use crate::{msg::{FactoryHandleMsg, FactoryQueryMsg, HandleMsg, InitMsg, QueryMsg, Snip20Msg, StateResponse, IsKeyValidResponse}};
use crate::state::{config, config_read, State, ArenaState, Action, Class, Round, FactoryInfo};
use crate::combat::{round_result};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        arena_name: msg.name,
        secret_entropy: vec![msg.entropy.to_be_bytes()],
        arena_state: ArenaState {
            tokens_locked: Uint128::from(0 as u128),
            player1: None,
            player1_class_name: None,
            player2: None,
            player2_class_name: None,
            rounds: vec![gen_empty_rounds(1),gen_empty_rounds(2),gen_empty_rounds(3),gen_empty_rounds(4),gen_empty_rounds(5)],
        },
        known_snip_20: vec![msg.snip20_contract_code_address.clone()],
        classes: vec![
            Class {
                name: "Warrior".to_string(),
                base_hp: 125,
                base_attack: 20,
                base_dodge_chance: 0,
                actions: vec![
                 Action {
                    name: "Attack".to_string(),
                    preparation_needed: false
                },
                 Action {
                    name: "UnblockableAttack".to_string(),
                    preparation_needed: true,
                },
                 Action {
                    name: "Block".to_string(),
                    preparation_needed: false,
                },
                 Action {
                    name: "CounterAttack".to_string(),
                    preparation_needed: true,
                },
                 Action {
                    name: "Prepare".to_string(),
                    preparation_needed: false
                }]
            }, 
        ],  
        factory: FactoryInfo {
            contract_address: msg.factory_address.clone(),
            contract_hash: msg.factory_hash.clone(),
            auth_key: msg.factory_key.clone()
        }
    };

    config(&mut deps.storage).save(&state)?;

    // send callback to factory
    let callback_msg = FactoryHandleMsg::InitCallBackFromArenaToFactory {
        auth_key: msg.factory_key.clone(),
        contract_address: env.contract.address
    };

    let cosmos_msg = callback_msg.to_cosmos_msg(msg.factory_hash.clone(), msg.factory_address.clone(), None)?;
    
    // send register to snip20
    let snip20_register_msg = to_binary(&Snip20Msg::register_receive(env.contract_code_hash))?;
    let token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: msg.snip20_contract_code_address.clone(),
        callback_code_hash: msg.snip20_contract_code_hash,
        msg: snip20_register_msg,
        send: vec![],
    });
    
    Ok(InitResponse {
        messages: vec![
            token_msg,
            cosmos_msg,
        ],
        log: vec![],
    })
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        // SNIP20 functions: tokens to interact with this contract
        HandleMsg::Receive { sender, from, amount, msg } => try_receive(deps, env, sender, from, amount, msg),
        // Base functions
        // Receive => JoinArena()
        HandleMsg::RoundAction { action_name } => round_action(deps, env, action_name),
        _ => Err(StdError::generic_err("Handler not found!"))
    }
}

fn gen_empty_rounds(round_number: i32) -> Round {
    return Round {
        number: round_number,
        status: "Empty".to_string(),
        player1_hp: None,
        player1_preparation: Some(false),
        player2_hp: None,
        player2_preparation: Some(false),
        player1_action_name: None,
        player1_attack_level: None,
        player2_action_name: None,
        player2_attack_level: None,
    }
}

fn can_deposit(amount: Uint128, bet_size: Uint128) -> StdResult<Uint128> {
    if amount.u128() != bet_size.u128() {
        return Err(StdError::generic_err(format!(
            "{} bet diferent {}",
            amount,
            bet_size
        )));
    }

    Ok(amount)
}

pub fn round_action<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    action_name: String
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    // check room and i'm one of the players on that room
    let mut player_number: i32 = 0;
    if state.arena_state.player1 == Some(env.message.sender.clone()) { player_number = 1 }
    if state.arena_state.player2 == Some(env.message.sender.clone()) { player_number = 2 }
    if player_number == 0 {
        return Err(StdError::generic_err(format!(
            "Your not a participant on this arena: {}",
            state.arena_name
        )));
    }

    // Check current round status and player action
    let latest_round = state.arena_state.rounds.iter().rev().find(|round| round.status == "WaitingActions".to_string());
    if latest_round == None {
        return Err(StdError::generic_err(format!(
            "No rounds on this arena: {}",
            state.arena_name
        )));
    }

    if player_number == 1 && !(latest_round.unwrap().player1_action_name == None ) {
        return Err(StdError::generic_err(format!(
            "You already submited an action for this round: {}, {:?}",
            latest_round.unwrap().number,
            latest_round.unwrap().player1_action_name
        )));
    }
    if player_number == 2 && !(latest_round.unwrap().player2_action_name == None ) {
        return Err(StdError::generic_err(format!(
            "You already submited an action for this round: {}, {:?}",
            latest_round.unwrap().number,
            latest_round.unwrap().player2_action_name
        )));
    }

    let player1_class = state.classes.iter().find(|class| class.name == state.arena_state.player1_class_name.clone().unwrap().to_string()).unwrap();
    let player2_class = state.classes.iter().find(|class| class.name == state.arena_state.player2_class_name.clone().unwrap().to_string()).unwrap();
    let mut player1_action = None;
    let mut player2_action = None;

    // check action_name valid
    if player_number == 1 {
        player1_action = player1_class.actions.iter().find(|action| action.name == action_name && action.preparation_needed == latest_round.unwrap().player1_preparation.unwrap());
        if player1_action == None {
            return Err(StdError::generic_err(format!(
                "Invalid action_name: {}",
                action_name
            )));
        }
        if latest_round.unwrap().player2_action_name != None {
            player2_action = player2_class.actions.iter().find(|action| action.name == latest_round.unwrap().player2_action_name.clone().unwrap() && action.preparation_needed == latest_round.unwrap().player2_preparation.unwrap());
        }
    }

    if player_number == 2 {
        player2_action = player2_class.actions.iter().find(|action| action.name == action_name && action.preparation_needed == latest_round.unwrap().player2_preparation.unwrap());
        if player2_action == None {
            return Err(StdError::generic_err(format!(
                "Invalid action_name: {}",
                action_name
            )));
        }
        if latest_round.unwrap().player1_action_name != None {
            player1_action = player1_class.actions.iter().find(|action| action.name == latest_round.unwrap().player1_action_name.clone().unwrap() && action.preparation_needed == latest_round.unwrap().player1_preparation.unwrap());
        }
    }
    
    //random generate level of action
    let mut hasher = Sha256::new();
    state.secret_entropy.iter().for_each(|el| hasher.update(el));
    hasher.update(state.arena_name.as_bytes());
    hasher.update(&latest_round.unwrap().number.to_be_bytes());
    hasher.update(&player_number.to_be_bytes());
    let seed:[u8; 32] = hasher.finalize().into();
    let mut rng = ChaChaRng::from_seed(seed);
    let level = rng.gen_range(1,3);

    config(&mut deps.storage).update(|mut state| {
        let mut rounds = state.arena_state.rounds;
        let cur_round_index = (latest_round.unwrap().number - 1) as usize;
        let next_round_index = (latest_round.unwrap().number) as usize;

        if player_number == 1 {
            rounds[cur_round_index].player1_action_name = Some(action_name);
            rounds[cur_round_index].player1_attack_level = Some(level);
        } else if player_number == 2 {
            rounds[cur_round_index].player2_action_name = Some(action_name);
            rounds[cur_round_index].player2_attack_level = Some(level);
        }

        //if we have both player actions => do the HP and Stamina calculations => create new round
        if rounds[cur_round_index].player1_action_name != None && rounds[cur_round_index].player2_action_name != None {
            let (player1_hp, player1_preparation, player2_hp, player2_preparation) = round_result(
                rounds[cur_round_index].player1_hp.unwrap(), 
                rounds[cur_round_index].player1_preparation.unwrap(),
                rounds[cur_round_index].player2_hp.unwrap(), 
                rounds[cur_round_index].player2_preparation.unwrap(),
                player1_action.unwrap(), 
                player2_action.unwrap(), 
                player1_class, 
                player2_class
            );
            if player1_hp > 0 || player2_hp > 0 {
                rounds[next_round_index].status = "WaitingActions".to_string();
                rounds[next_round_index].player1_hp = Some(player1_hp);
                rounds[next_round_index].player2_hp = Some(player2_hp);
                rounds[next_round_index].player1_preparation = Some(player1_preparation);
                rounds[next_round_index].player2_preparation = Some(player2_preparation);
            }
            rounds[cur_round_index].status = "Completed".to_string();
        }

        state.arena_state.rounds = rounds;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn try_receive<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _sender: HumanAddr,
    from: HumanAddr,
    amount: Uint128,
    msg: Binary,
) -> StdResult<HandleResponse> {
    let msg: HandleMsg = from_binary(&msg)?;

    if matches!(msg, HandleMsg::Receive { .. }) {
        return Err(StdError::generic_err(
            "Recursive call to receive() is not allowed",
        ));
    }

    let state = config_read(&deps.storage).load()?;
    if !state.known_snip_20.contains(&env.message.sender) {
        return Err(StdError::generic_err(format!(
            "{} is not a known SNIP-20 coin that this contract registered to",
            env.message.sender
        )));
    }
    
    if let HandleMsg::JoinArena { class_name , secret } = msg.clone() {
        return join_arena(deps, env, class_name, secret, from, amount)
    } else {
        return Err(StdError::generic_err(format!(
            "Receive handler not found!"
        )));
    }
}

pub fn join_arena<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    class_name: String,
    secret: u64,
    from: HumanAddr,
    amount: Uint128
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    if state.arena_state.player1 != None && state.arena_state.player2 != None {
        return Err(StdError::generic_err(format!(
            "Arena full: {}",
            state.arena_name
        )));
    }

    let mut deposit = amount;
    // check bet if player 2
    if state.arena_state.player1 != None {
        deposit = can_deposit(amount, state.arena_state.tokens_locked)?;
    }
    
    // check class_name valid
    if !state.classes.iter().any(|class| class.name == class_name) {
        return Err(StdError::generic_err(format!(
            "Invalid class_name: {}",
            class_name
        )));
    }

    //check if not the same player
    if state.arena_state.player1 != None && state.arena_state.player1 == Some(from.clone()) {
        return Err(StdError::generic_err(format!(
            "Arena full: {}",
            state.arena_name
        )));
    }

    config(&mut deps.storage).update(|mut state| {
        let mut arena = state.arena_state;
        // Check if I'm player 1 or 2
        if arena.player1 == None {
            arena.player1 = Some(from);
            arena.player1_class_name = Some(class_name);
            arena.tokens_locked = Uint128::from(deposit);
        } else {
            if arena.player2 == None {
                let player1_class = state.classes.iter().find(|class| class.name == arena.player1_class_name.clone().unwrap());
                let player2_class = state.classes.iter().find(|class| class.name == class_name);
                arena.player2 = Some(from);
                arena.player2_class_name = Some(class_name);
                arena.tokens_locked = arena.tokens_locked + deposit;
                arena.rounds[0].player1_hp = Some(player1_class.unwrap().base_hp);
                arena.rounds[0].player2_hp = Some(player2_class.unwrap().base_hp);
                arena.rounds[0].player1_preparation = Some(false);
                arena.rounds[0].player2_preparation = Some(false);
                arena.rounds[0].status = "WaitingActions".to_string()
            }
        }
        state.arena_state = arena;
        state.secret_entropy.push(secret.to_be_bytes());
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {user_address,user_viewkey } => to_binary(&get_state(deps, user_address, user_viewkey)?),
    }
}

fn get_state<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    user_address: HumanAddr,
    user_viewkey: String
) -> StdResult<StateResponse> {
    let state = config_read(&deps.storage).load()?;

    let response: IsKeyValidResponse =
    FactoryQueryMsg::IsKeyValid {
        factory_key: state.factory.auth_key.clone(),
        viewing_key: user_viewkey.clone(),
        address: user_address.clone()
    }.query(&deps.querier, state.factory.contract_hash.clone(), state.factory.contract_address.clone())?;
    
    if response.is_key_valid.is_valid {
        return Ok(StateResponse {
            state: state.clone()
            /*bet_size: state.bet_size,
            classes: state.classes,
            arena: ArenaPreview {
                name: state.arena_name,
                num_players,
                num_rounds,
            },*/
        })
    } else {
        return Err(StdError::generic_err(format!(
            "Invalid address - viewkey pair!"
        ))); 
    }
    /*let mut num_players: i32 = 0;
    let mut num_rounds: i32 = 0;

    if state.arena_state.player1 != None { num_players = num_players + 1}
    if state.arena_state.player2 != None { num_players = num_players + 1}

    let latest_round = state.arena_state.rounds.iter().rev().find(|round| round.status == "WaitingActions".to_string());
    if latest_round != None {num_rounds = latest_round.unwrap().number }
*/

}