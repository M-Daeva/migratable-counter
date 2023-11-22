#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use counter_base::{
    counter_new::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    error::ContractError,
};

use crate::actions::{
    execute::{try_create_counter, try_set_counter, try_update_counter},
    instantiate::try_instantiate,
    other::migrate_contract,
    query::{query_counters, query_total_calls, query_total_calls_previous},
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
        ExecuteMsg::CreateCounter {} => try_create_counter(deps, env, info),
        ExecuteMsg::SetCounter { value } => try_set_counter(deps, env, info, value),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryCounters { addresses } => {
            to_json_binary(&query_counters(deps, env, addresses)?)
        }
        QueryMsg::QueryTotalCalls {} => to_json_binary(&query_total_calls(deps, env)?),
        QueryMsg::QueryTotalCallsPrevious {} => {
            to_json_binary(&query_total_calls_previous(deps, env)?)
        }
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
