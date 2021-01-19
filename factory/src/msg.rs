use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub entropy: String,
    pub arena_contracts_code_id: u64,
    pub arena_contracts_code_hash: String,
    pub snip20_contract_code_address: HumanAddr,
    pub snip20_contract_code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ArenaContractInitMsg {
    pub name: String,
    pub entropy: u64,
    pub factory_address: HumanAddr,
    pub factory_hash: String,
    pub factory_key: String,
    pub snip20_contract_code_address: HumanAddr,
    pub snip20_contract_code_hash: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateViewingKey {entropy: String},
    ChangeArenaContractCodeId {code_id: u64, code_hash: String},
    NewArenaInstanciate {name: String, entropy: u64},
    InitCallBackFromArenaToFactory {auth_key: String, contract_address: HumanAddr}
}

/// success or failure response
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum ResponseStatus {
    Success,
    Failure,
}
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    /// response from creating a viewing key
    ViewingKey { key: String },
    /// generic status response
    Status {
        /// success or failure
        status: ResponseStatus,
        /// execution description
        #[serde(skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}
/// Queries
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// authenticates the supplied address/viewing key.  This should only be called by arenas
    IsKeyValid {
        /// address whose viewing key is being authenticated
        address: HumanAddr,
        /// viewing key
        viewing_key: String,
        //authentication on factory functions
        factory_key: String
    },
    ArenaContractCodeId {},
    Arenas {}
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    /// Viewing Key Error
    ViewingKeyError { error: String },
    /// result of authenticating address/key pair
    IsKeyValid { is_valid: bool },
    ArenaContractCodeID {code_id: u64, code_hash: String},
    Arenas {arenas: Vec<HumanAddr>}
}
