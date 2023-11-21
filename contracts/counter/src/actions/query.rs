use cosmwasm_std::{Addr, Deps, Env, Order, StdResult, Uint128};

use counter_base::counter::state::{COUNTERS, TOTAL_CALLS};

pub fn query_counters(deps: Deps, _env: Env) -> StdResult<Vec<(Addr, Uint128)>> {
    Ok(COUNTERS
        .range(deps.storage, None, None, Order::Ascending)
        .flatten()
        .collect())
}

pub fn query_total_calls(deps: Deps, _env: Env) -> StdResult<Uint128> {
    Ok(Uint128::from(TOTAL_CALLS.load(deps.storage)?))
}

pub fn query_total_calls_previous(_deps: Deps, _env: Env) -> StdResult<Uint128> {
    Ok(Uint128::zero())
}
