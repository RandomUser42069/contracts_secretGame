use cosmwasm_std::{Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResponse, Querier, QueryResult, ReadonlyStorage, StdError, StdResult, Storage, Uint128, to_binary};

use crate::msg::{ArenaContractInitMsg, HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg, ResponseStatus::Success,};
use crate::rand::sha_256;
use crate::state::{save, load, may_load};
use crate::viewing_key::{ViewingKey, VIEWING_KEY_SIZE};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};

use secret_toolkit::{
    utils::{InitCallback},
};

/// prefix for viewing keys
pub const PREFIX_VIEW_KEY: &[u8] = b"viewingkey";
/// storage key for prng seed
pub const PRNG_SEED_KEY: &[u8] = b"prngseed";
/// storage key for the factory admin
pub const ADMIN_KEY: &[u8] = b"admin";
/// storage key for the children contracts 
pub const SNIP20_CONTRACT_CODE_ADDRESS: &[u8] = b"snip20contractcodeaddress";
/// storage key for the children contracts 
pub const SNIP20_CONTRACT_CODE_HASH: &[u8] = b"snip20contractcodehash";
/// storage key for the children contracts 
pub const ARENA_CONTRACT_CODE_ID: &[u8] = b"arenacontractscodeid";
/// storage key for the children contracts 
pub const ARENA_CONTRACT_CODE_HASH: &[u8] = b"arenacontractscodehash";
/// storage key for the children contracts 
pub const ARENA_CONTRACTS: &[u8] = b"arenacontracts";
/// storage key for the factory admin
pub const FACTORY_KEY: &[u8] = b"factorykey";
/// response size
pub const BLOCK_SIZE: usize = 256;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let prng_seed: Vec<u8> = sha_256(base64::encode(msg.entropy.clone()).as_bytes()).to_vec();
    let key = ViewingKey::new(&env, &prng_seed, msg.entropy.clone().as_ref());
    let arena_contracts: Vec<HumanAddr> = vec![];
    save(&mut deps.storage, FACTORY_KEY, &format!("{}", key))?;
    save(&mut deps.storage, PRNG_SEED_KEY, &prng_seed)?;
    save(&mut deps.storage, ADMIN_KEY, &env.message.sender)?;
    save(&mut deps.storage, ARENA_CONTRACT_CODE_ID, &msg.arena_contracts_code_id)?;
    save(&mut deps.storage, ARENA_CONTRACT_CODE_HASH, &msg.arena_contracts_code_hash)?;
    save(&mut deps.storage, SNIP20_CONTRACT_CODE_ADDRESS, &msg.snip20_contract_code_address)?;
    save(&mut deps.storage, SNIP20_CONTRACT_CODE_HASH, &msg.snip20_contract_code_hash)?;
    save(&mut deps.storage, ARENA_CONTRACTS, &arena_contracts)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateViewingKey { entropy } => try_create_key(deps, env, &entropy),
        HandleMsg::ChangeArenaContractCodeId { code_id, code_hash } => try_change_arena_contract_code_id(deps, env, &code_id, &code_hash),
        HandleMsg::NewArenaInstanciate {name, entropy} => try_arena_instanciate(deps, env, &name, &entropy),
        HandleMsg::InitCallBackFromArenaToFactory {auth_key, contract_address} => try_arena_instanciated_callback(deps, env, auth_key, contract_address)
    }
}

fn try_create_key<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    entropy: &str,
) -> HandleResult {
    // create and store the key
    let prng_seed: Vec<u8> = load(&deps.storage, PRNG_SEED_KEY)?;
    let key = ViewingKey::new(&env, &prng_seed, entropy.as_ref());
    let message_sender = &deps.api.canonical_address(&env.message.sender)?;
    let mut key_store = PrefixedStorage::new(PREFIX_VIEW_KEY, &mut deps.storage);
    save(&mut key_store, message_sender.as_slice(), &key.to_hashed())?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::ViewingKey {
            key: format!("{}", key),
        })?),
    })
}

fn try_change_arena_contract_code_id<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    code_id: &u64,
    code_hash: &String
) -> HandleResult {
    let admin: HumanAddr = load(&deps.storage, ADMIN_KEY)?;
    if env.message.sender != admin {
        return Err(StdError::generic_err(
            "Permission Denied.",
        ));
    }
    
    save(&mut deps.storage, ARENA_CONTRACT_CODE_ID, &code_id)?;
    save(&mut deps.storage, ARENA_CONTRACT_CODE_HASH, &code_hash)?;
    
    Ok(HandleResponse::default())
}

fn try_arena_instanciate<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    name: &String,
    entropy: &u64,
) -> HandleResult {  
    let snip20_contract_code_address: HumanAddr = load(&deps.storage, SNIP20_CONTRACT_CODE_ADDRESS)?;
    let snip20_contract_code_hash: String = load(&deps.storage, SNIP20_CONTRACT_CODE_HASH)?; 
    let arena_contract_code_id: u64 = load(&deps.storage, ARENA_CONTRACT_CODE_ID)?;
    let arena_contract_code_hash: String = load(&deps.storage, ARENA_CONTRACT_CODE_HASH)?;
    let factory_key: String = load(&deps.storage, FACTORY_KEY)?;
    
    let initmsg = ArenaContractInitMsg {
        name: name.clone(),
        entropy: entropy.clone(),
        factory_hash: env.contract_code_hash,
        factory_address: env.contract.address,
        factory_key,
        snip20_contract_code_address,
        snip20_contract_code_hash
    };
    impl InitCallback for ArenaContractInitMsg {
        const BLOCK_SIZE: usize = BLOCK_SIZE;
    }
    let cosmosmsg =
        initmsg.to_cosmos_msg(format!("{} {}", "Arena - ", name).to_string(), arena_contract_code_id, arena_contract_code_hash, None)?;

    Ok(HandleResponse {
        messages: vec![cosmosmsg],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::Status {
            status: Success,
            message: None,
        })?),
    })
}

pub fn try_arena_instanciated_callback<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    auth_key: String,
    contract_address: HumanAddr
) -> HandleResult {   
    let mut arena_contracts: Vec<HumanAddr> = load(&deps.storage, ARENA_CONTRACTS)?;
    let factory_key: String = load(&deps.storage, FACTORY_KEY)?;
    let input_key: String = auth_key;
    
    if factory_key != input_key {
        return Err(StdError::generic_err(
            "Permission Denied.",
        ));
    }

    arena_contracts.insert(0, contract_address);
    save(&mut deps.storage, ARENA_CONTRACTS, &arena_contracts)?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::IsKeyValid {
            address,
            viewing_key,
            factory_key
        } => try_validate_key(deps, &address, viewing_key, factory_key),
        QueryMsg::ArenaContractCodeId {} => arena_contract_code_id(deps),
        QueryMsg::Arenas {} => arenas(deps)
    }
}

fn try_validate_key<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: &HumanAddr,
    viewing_key: String,
    factory_key: String
) -> QueryResult {
    let addr_raw = &deps.api.canonical_address(address)?;
    let state_factory_key: String = load(&deps.storage, FACTORY_KEY)?;
    if factory_key != state_factory_key {
        return Err(StdError::generic_err(
            "Permission Denied.",
        ));
    }

    to_binary(&QueryAnswer::IsKeyValid {
        is_valid: is_key_valid(&deps.storage, addr_raw, viewing_key)?,
    })
}

fn is_key_valid<S: ReadonlyStorage>(
    storage: &S,
    address: &CanonicalAddr,
    viewing_key: String,
) -> StdResult<bool> {
    // load the address' key
    let read_key = ReadonlyPrefixedStorage::new(PREFIX_VIEW_KEY, storage);
    let load_key: Option<[u8; VIEWING_KEY_SIZE]> = may_load(&read_key, address.as_slice())?;
    let input_key = ViewingKey(viewing_key);
    // if a key was set
    if let Some(expected_key) = load_key {
        // and it matches
        if input_key.check_viewing_key(&expected_key) {
            return Ok(true);
        }
    } else {
        // Checking the key will take significant time. We don't want to exit immediately if it isn't set
        // in a way which will allow to time the command and determine if a viewing key doesn't exist
        input_key.check_viewing_key(&[0u8; VIEWING_KEY_SIZE]);
    }
    Ok(false)
}


fn arena_contract_code_id <S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> QueryResult {
    let arena_contract_code_id: u64 = load(&deps.storage, ARENA_CONTRACT_CODE_ID)?;
    let arena_contract_code_hash: String = load(&deps.storage, ARENA_CONTRACT_CODE_HASH)?;
    to_binary(&QueryAnswer::ArenaContractCodeID {
        code_id: arena_contract_code_id,
        code_hash: arena_contract_code_hash
    })
}

fn arenas <S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> QueryResult {
    let arenas: Vec<HumanAddr> = load(&deps.storage, ARENA_CONTRACTS)?;
    to_binary(&QueryAnswer::Arenas {
        arenas
    })
}