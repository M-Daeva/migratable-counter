#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use counter_base::{
    counter::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    error::ContractError,
};

use crate::actions::{
    execute::{try_create_counter, try_reset_counter, try_update_counter},
    instantiate::try_instantiate,
    other::migrate_contract,
    query::{query_config, query_counters},
};

/// Creates a new contract with the specified parameters packed in the "msg" variable
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    try_instantiate(deps, env, info, msg)
}

/// Exposes all the execute functions available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateCounter { action_type, value } => {
            try_update_counter(deps, env, info, action_type, value)
        }
        ExecuteMsg::ResetCounter {} => try_reset_counter(deps, env, info),
        ExecuteMsg::CreateCounter {} => try_create_counter(deps, env, info),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_json_binary(&query_config(deps, env)?),
        QueryMsg::QueryCounters {} => to_json_binary(&query_counters(deps, env)?),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
