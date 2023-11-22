use cosmwasm_std::{Addr, Deps, Env, Order, StdResult, Uint128};

use counter_base::{
    counter_new::{
        msg::QueryCountersResponse,
        state::{COUNTERS, TOTAL_CALLS, TOTAL_CALLS_PREVIOUS},
    },
    utils::filter_by_address_list,
};

pub fn query_counters(
    deps: Deps,
    _env: Env,
    addresses: Option<Vec<String>>,
) -> StdResult<Vec<QueryCountersResponse>> {
    let counters: Vec<(Addr, Uint128)> = COUNTERS
        .range(deps.storage, None, None, Order::Ascending)
        .flatten()
        .collect();

    Ok(filter_by_address_list(deps.api, &addresses, &counters)?
        .into_iter()
        .map(|(address, counter)| QueryCountersResponse {
            owner: address,
            counter_value: counter,
        })
        .collect())
}

pub fn query_total_calls(deps: Deps, _env: Env) -> StdResult<Uint128> {
    TOTAL_CALLS.load(deps.storage)
}

pub fn query_total_calls_previous(deps: Deps, _env: Env) -> StdResult<Uint128> {
    TOTAL_CALLS_PREVIOUS.load(deps.storage)
}
