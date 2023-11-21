use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use counter_base::{
    counter::{
        msg::InstantiateMsg,
        state::{CONFIG, CONTRACT_NAME, TOTAL_CALLS},
        types::Config,
    },
    error::ContractError,
    utils::Attrs,
};

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let attrs = Attrs::init("try_instantiate");

    CONFIG.save(deps.storage, &Config::new(&info.sender))?;

    TOTAL_CALLS.save(deps.storage, &0)?;

    Ok(Response::new().add_attributes(attrs))
}
