use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::counter_new::types::ActionType;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateCounter {},

    UpdateCounter {
        action_type: ActionType,
        value: Uint128,
    },

    SetCounter {
        value: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<QueryCountersResponse>)]
    QueryCounters { addresses: Option<Vec<String>> },

    #[returns(cosmwasm_std::Uint128)]
    QueryTotalCalls {},

    #[returns(cosmwasm_std::Uint128)]
    QueryTotalCallsPrevious {},

    #[returns(cosmwasm_std::Uint128)]
    QueryTotalDeposited {},
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub struct QueryCountersResponse {
    pub owner: Addr,
    pub counter_value: Uint128,
}
