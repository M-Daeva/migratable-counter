use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::counter::types::ActionType;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateCounter {},

    UpdateCounter {
        action_type: ActionType,
        value: Uint128,
    },

    // TODO: 1. Remove it and add SetCounter { value: Uint128 }
    ResetCounter {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // TODO: 2. Update to QueryCounters { addresses: Option<Vec<String>> } -> Vec<QueryCountersResponse>
    // and implement `struct QueryCountersResponse { owner: Addr, counter_value: Uint128 }`
    #[returns(Vec<(cosmwasm_std::Addr, cosmwasm_std::Uint128)>)]
    QueryCounters {},

    #[returns(cosmwasm_std::Uint128)]
    QueryTotalCalls {},

    #[returns(cosmwasm_std::Uint128)]
    QueryTotalCallsPrevious {},
}
