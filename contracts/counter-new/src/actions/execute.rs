use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

use counter_base::{
    counter_new::{
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

    TOTAL_CALLS.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x + Uint128::one())
    })?;

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
        ActionType::Mul => {
            counter *= value;
        }
    }

    COUNTERS.save(deps.storage, &info.sender, &counter)?;

    TOTAL_CALLS.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x + Uint128::one())
    })?;

    Ok(Response::new().add_attributes([("action", "try_update_counter")]))
}

pub fn try_set_counter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    value: Uint128,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    COUNTERS.load(deps.storage, &info.sender)?;

    COUNTERS.save(deps.storage, &info.sender, &value)?;

    TOTAL_CALLS.update(deps.storage, |x| -> StdResult<Uint128> {
        Ok(x + Uint128::one())
    })?;

    Ok(Response::new().add_attributes([("action", "try_set_counter")]))
}
