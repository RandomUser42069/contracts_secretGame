use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, HumanAddr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub arena_name: String,
    pub known_snip_20: Vec<HumanAddr>,
    pub secret_entropy: Vec<[u8; 8]>,
    pub arena_state: ArenaState,
    pub classes: Vec<Class>,
    pub factory: FactoryInfo
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FactoryInfo {
    pub contract_address: HumanAddr,
    pub contract_hash: String,
    pub auth_key: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Statistics {
    pub total_finished_brawls: i32,
    pub total_betted: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Class {
    pub name: String,
    pub base_hp: i32,
    pub base_attack: i32,
    pub base_dodge_chance: i32,
    pub actions: Vec<Action>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Action {
    pub name: String,
    pub preparation_needed: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Level {
    pub number: i32,
    pub bonus: i32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ArenaState {
    pub tokens_locked: Uint128,
    pub player1: Option<HumanAddr>,
    pub player1_class_name:Option<String>,
    pub player2: Option<HumanAddr>,
    pub player2_class_name: Option<String>,
    pub rounds: Vec<Round>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Round {
    pub number: i32,
    pub status: String, //Empty => Waitingc => Completed
    pub player1_hp: Option<i32>,
    pub player1_preparation: Option<bool>,
    pub player2_hp: Option<i32>,
    pub player2_preparation: Option<bool>,
    pub player1_action_name: Option<String>,
    pub player1_attack_level: Option<i32>,
    pub player2_action_name: Option<String>,
    pub player2_attack_level: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ArenaPreview {
    pub name: String,
    pub num_players: i32,
    pub num_rounds: i32
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
