use cosmwasm_std::{coin, Addr, StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use counter_base::{
    counter::{
        msg::{ExecuteMsg, QueryMsg},
        types::{ActionType, Config},
    },
    error::parse_err,
};

use crate::helpers::suite::{
    core::Project,
    types::{ProjectAccount, ProjectCoin},
};

pub trait CounterExtension {
    fn counter_try_create_counter(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse>;

    fn counter_try_update_counter(
        &mut self,
        sender: ProjectAccount,
        action_type: ActionType,
        value: u128,
    ) -> StdResult<AppResponse>;

    fn counter_try_reset_counter(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn counter_query_config(&self) -> StdResult<Config>;

    fn counter_query_counters(&self) -> StdResult<Vec<(Addr, Uint128)>>;
}

impl CounterExtension for Project {
    #[track_caller]
    fn counter_try_create_counter(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_counter_address(),
                &ExecuteMsg::CreateCounter {},
                &[coin(amount, asset.to_string())],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn counter_try_update_counter(
        &mut self,
        sender: ProjectAccount,
        action_type: ActionType,
        value: u128,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_counter_address(),
                &ExecuteMsg::UpdateCounter {
                    action_type,
                    value: Uint128::new(value),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn counter_try_reset_counter(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_counter_address(),
                &ExecuteMsg::ResetCounter {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn counter_query_config(&self) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_counter_address(), &QueryMsg::QueryConfig {})
    }

    #[track_caller]
    fn counter_query_counters(&self) -> StdResult<Vec<(Addr, Uint128)>> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_counter_address(), &QueryMsg::QueryCounters {})
    }
}
