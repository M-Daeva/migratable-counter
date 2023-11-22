use cosmwasm_std::{DepsMut, Env, Response, StdError, Uint128};
use cw2::{get_contract_version, set_contract_version};

use semver::Version;

use counter_base::{
    counter::state as state_previous,
    counter_new::{msg::MigrateMsg, state as state_new},
    error::ContractError,
};

pub fn migrate_contract(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let version_previous: Version = get_contract_version(deps.storage)?
        .version
        .parse()
        .map_err(|_| StdError::generic_err("Parsing previous version error"))?;

    let version_new: Version = env!("CARGO_PKG_VERSION")
        .parse()
        .map_err(|_| StdError::generic_err("Parsing new version error"))?;

    if version_new > version_previous {
        set_contract_version(
            deps.storage,
            state_new::CONTRACT_NAME,
            version_new.to_string(),
        )?;

        // COUNTERS storage will be updated automatically.
        // For TOTAL_CALLS we have following changes:
        // 1) TOTAL_CALLS<u32> -> TOTAL_CALLS_PREVIOUS<Uint128>
        // 2) 0 -> TOTAL_CALLS<Uint128>
        let total_calls = state_previous::TOTAL_CALLS.load(deps.storage)?;
        state_new::TOTAL_CALLS_PREVIOUS.save(deps.storage, &Uint128::from(total_calls))?;
        state_new::TOTAL_CALLS.save(deps.storage, &Uint128::zero())?;
    }

    Ok(Response::new())
}
