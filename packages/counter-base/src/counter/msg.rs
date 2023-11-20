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

    // TODO: remove it and add SetCounter { value: Uint128 }
    ResetCounter {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // TODO: update to QueryCounters { addresses: Option<Vec<String>> } -> Vec<QueryCountersResponse>
    // and implement `struct QueryCountersResponse { owner: Addr, counter_value: Uint128 }`
    #[returns(Vec<(cosmwasm_std::Addr, cosmwasm_std::Uint128)>)]
    QueryCounters {},

    #[returns(crate::counter::types::Config)]
    QueryConfig {},
    // TODO: add QueryTotalDeposited {} -> Uint128
}

#[cw_serde]
pub enum MigrateMsg {}
