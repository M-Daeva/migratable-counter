use cosmwasm_std::{Addr, Deps, Env, Order, StdResult, Uint128};

use counter_base::counter::{
    state::{CONFIG, COUNTERS},
    types::Config,
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_counters(deps: Deps, _env: Env) -> StdResult<Vec<(Addr, Uint128)>> {
    Ok(COUNTERS
        .range(deps.storage, None, None, Order::Ascending)
        .flatten()
        .collect())
}
