use cosmwasm_std::{coin, Addr, StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use counter_base::{
    counter::{
        msg::{ExecuteMsg, QueryMsg},
        types::ActionType,
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
        funds: Option<(u128, ProjectCoin)>,
    ) -> StdResult<AppResponse>;

    fn counter_try_update_counter(
        &mut self,
        sender: ProjectAccount,
        action_type: ActionType,
        value: u128,
    ) -> StdResult<AppResponse>;

    fn counter_try_reset_counter(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn counter_query_counters(&self) -> StdResult<Vec<(Addr, Uint128)>>;

    fn counter_query_total_calls(&self) -> StdResult<Uint128>;

    fn counter_query_total_calls_previous(&self) -> StdResult<Uint128>;
}

impl CounterExtension for Project {
    #[track_caller]
    fn counter_try_create_counter(
        &mut self,
        sender: ProjectAccount,
        funds: Option<(u128, ProjectCoin)>,
    ) -> StdResult<AppResponse> {
        let send_funds = funds.map_or(vec![], |(amount, asset)| {
            vec![coin(amount, asset.to_string())]
        });

        self.app
            .execute_contract(
                sender.into(),
                self.get_counter_address(),
                &ExecuteMsg::CreateCounter {},
                &send_funds,
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
    fn counter_query_counters(&self) -> StdResult<Vec<(Addr, Uint128)>> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_counter_address(), &QueryMsg::QueryCounters {})
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
}
