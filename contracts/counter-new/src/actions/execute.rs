use cosmwasm_std::{
    to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, Uint128,
    WasmMsg,
};

use counter_base::{
    counter_new::{
        msg::ExecuteMsg,
        state::{
            COUNTERS, SET_COUNTER_QUEUE, SET_COUNTER_REPLY_ID, TOTAL_CALLS, UPDATE_COUNTER_QUEUE,
            UPDATE_COUNTER_REPLY_ID,
        },
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
    env: Env,
    info: MessageInfo,
    action_type: ActionType,
    value: Uint128,
) -> Result<Response, ContractError> {
    // if counter exists update counter
    if COUNTERS.has(deps.storage, &info.sender) {
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

        return Ok(Response::new().add_attributes([("action", "try_update_counter")]));
    }

    // if counter doesn't exist create it then update counter
    let funds = one_coin(&info)?;

    UPDATE_COUNTER_QUEUE.push_back(deps.storage, &(info, action_type, value))?;

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&ExecuteMsg::CreateCounter {})?,
        funds: vec![funds],
    });

    let submsg = SubMsg::reply_on_success(msg, UPDATE_COUNTER_REPLY_ID);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attributes([("action", "try_update_counter")]))
}

pub fn try_set_counter(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    value: Uint128,
) -> Result<Response, ContractError> {
    // if counter exists set counter
    if COUNTERS.has(deps.storage, &info.sender) {
        nonpayable(&info)?;

        COUNTERS.save(deps.storage, &info.sender, &value)?;

        TOTAL_CALLS.update(deps.storage, |x| -> StdResult<Uint128> {
            Ok(x + Uint128::one())
        })?;

        return Ok(Response::new().add_attributes([("action", "try_set_counter")]));
    }

    // if counter doesn't exist create it then set counter
    let funds = one_coin(&info)?;

    SET_COUNTER_QUEUE.push_back(deps.storage, &(info, value))?;

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&ExecuteMsg::CreateCounter {})?,
        funds: vec![funds],
    });

    let submsg = SubMsg::reply_on_success(msg, SET_COUNTER_REPLY_ID);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attributes([("action", "try_set_counter")]))
}
