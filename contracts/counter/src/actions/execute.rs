use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use counter_base::{
    counter::{
        state::{COUNTERS, TOTAL_CALLS},
        types::ActionType,
    },
    error::ContractError,
    utils::{nonpayable, one_coin},
};

pub fn try_create_counter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    one_coin(&info)?;

    COUNTERS.save(deps.storage, &info.sender, &Uint128::zero())?;

    TOTAL_CALLS.update(deps.storage, |x| -> StdResult<u32> { Ok(x + 1) })?;

    Ok(Response::new().add_attributes([("action", "try_create_counter")]))
}

pub fn try_update_counter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    action_type: ActionType,
    value: Uint128,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut counter = COUNTERS.load(deps.storage, &info.sender)?;

    match action_type {
        ActionType::Add => {
            counter += value;
        }
        ActionType::Sub => {
            counter -= value;
        }
    }

    COUNTERS.save(deps.storage, &info.sender, &counter)?;

    TOTAL_CALLS.update(deps.storage, |x| -> StdResult<u32> { Ok(x + 1) })?;

    Ok(Response::new().add_attributes([("action", "try_update_counter")]))
}

pub fn try_reset_counter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    COUNTERS.save(deps.storage, &info.sender, &Uint128::zero())?;

    TOTAL_CALLS.update(deps.storage, |x| -> StdResult<u32> { Ok(x + 1) })?;

    Ok(Response::new().add_attributes([("action", "try_reset_counter")]))
}
