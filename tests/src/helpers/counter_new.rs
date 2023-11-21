use cosmwasm_std::{coin, StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use counter_base::{
    counter_new::{
        msg::{ExecuteMsg, QueryCountersResponse, QueryMsg},
        types::ActionType,
    },
    error::parse_err,
};

use crate::helpers::suite::{
    core::Project,
    types::{ProjectAccount, ProjectCoin},
};

pub trait CounterNewExtension {
    fn counter_new_try_create_counter(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse>;

    fn counter_new_try_update_counter(
        &mut self,
        sender: ProjectAccount,
        action_type: ActionType,
        value: u128,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse>;

    fn counter_new_try_set_counter(
        &mut self,
        sender: ProjectAccount,
        value: u128,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse>;

    fn counter_query_counters(
        &self,
        addresses: Option<Vec<String>>,
    ) -> StdResult<QueryCountersResponse>;

    fn counter_query_total_calls(&self) -> StdResult<Uint128>;

    fn counter_query_total_calls_previous(&self) -> StdResult<Uint128>;

    fn counter_query_total_deposited(&self) -> StdResult<Uint128>;
}

impl CounterNewExtension for Project {
    #[track_caller]
    fn counter_new_try_create_counter(
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
    fn counter_new_try_update_counter(
        &mut self,
        sender: ProjectAccount,
        action_type: ActionType,
        value: u128,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_counter_address(),
                &ExecuteMsg::UpdateCounter {
                    action_type,
                    value: Uint128::new(value),
                },
                &[coin(amount, asset.to_string())],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn counter_new_try_set_counter(
        &mut self,
        sender: ProjectAccount,
        value: u128,
        amount: u128,
        asset: ProjectCoin,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_counter_address(),
                &ExecuteMsg::SetCounter {
                    value: Uint128::new(value),
                },
                &[coin(amount, asset.to_string())],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn counter_query_counters(
        &self,
        addresses: Option<Vec<String>>,
    ) -> StdResult<QueryCountersResponse> {
        let addresses = addresses
            .as_ref()
            .map(|x| x.iter().map(|y| y.to_string()).collect());

        self.app.wrap().query_wasm_smart(
            self.get_counter_address(),
            &QueryMsg::QueryCounters { addresses },
        )
    }

    #[track_caller]
    fn counter_query_total_calls(&self) -> StdResult<Uint128> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_counter_address(), &QueryMsg::QueryTotalCalls {})
    }

    #[track_caller]
    fn counter_query_total_calls_previous(&self) -> StdResult<Uint128> {
        self.app.wrap().query_wasm_smart(
            self.get_counter_address(),
            &QueryMsg::QueryTotalCallsPrevious {},
        )
    }

    #[track_caller]
    fn counter_query_total_deposited(&self) -> StdResult<Uint128> {
        self.app.wrap().query_wasm_smart(
            self.get_counter_address(),
            &QueryMsg::QueryTotalDeposited {},
        )
    }
}
