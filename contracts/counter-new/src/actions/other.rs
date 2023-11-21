use cosmwasm_std::{DepsMut, Env, Response};

use counter_base::{counter_new::msg::MigrateMsg, error::ContractError};

pub fn migrate_contract(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
