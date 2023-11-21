#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult,
};

use counter_base::{
    counter_new::{
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
        state::{
            SET_COUNTER_QUEUE, SET_COUNTER_REPLY_ID, UPDATE_COUNTER_QUEUE, UPDATE_COUNTER_REPLY_ID,
        },
    },
    error::ContractError,
    utils::unwrap_field,
};

use crate::actions::{
    execute::{try_create_counter, try_set_counter, try_update_counter},
    instantiate::try_instantiate,
    other::migrate_contract,
    query::{query_counters, query_total_calls, query_total_calls_previous, query_total_deposited},
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
        QueryMsg::QueryTotalDeposited {} => to_json_binary(&query_total_deposited(deps, env)?),
        QueryMsg::QueryTotalCalls {} => to_json_binary(&query_total_calls(deps, env)?),
        QueryMsg::QueryTotalCallsPrevious {} => {
            to_json_binary(&query_total_calls_previous(deps, env)?)
        }
    }
}

/// Exposes all the replies available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    let Reply { id, result: _ } = reply;

    match id {
        SET_COUNTER_REPLY_ID => {
            let (info, value) = unwrap_field(
                SET_COUNTER_QUEUE.pop_front(deps.storage)?,
                "SET_COUNTER_REPLY_ID",
            )?;
            try_set_counter(deps, env, info, value)
        }
        UPDATE_COUNTER_REPLY_ID => {
            let (info, action_type, value) = unwrap_field(
                UPDATE_COUNTER_QUEUE.pop_front(deps.storage)?,
                "UPDATE_COUNTER_REPLY_ID",
            )?;
            try_update_counter(deps, env, info, action_type, value)
        }
        _ => Err(ContractError::UndefinedReplyId),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
