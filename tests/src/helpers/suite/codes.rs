use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};

use serde::Serialize;

use counter_base::error::parse_err;

use crate::helpers::suite::{core::Project, types::ProjectAccount};

pub trait WithCodes {
    // store contracts
    fn store_counter_code(&mut self) -> u64;
    fn store_counter_new_code(&mut self) -> u64;

    // instantiate contracts
    fn instantiate_counter(&mut self, counter_code_id: u64) -> Addr;

    fn migrate_counter_new(
        &mut self,
        sender: ProjectAccount,
        counter_address: Addr,
        counter_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse>;
}

impl WithCodes for Project {
    fn store_counter_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            counter::contract::execute,
            counter::contract::instantiate,
            counter::contract::query,
        )))
    }

    fn store_counter_new_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                counter_new::contract::execute,
                counter_new::contract::instantiate,
                counter_new::contract::query,
            )
            .with_migrate(counter_new::contract::migrate),
        ))
    }

    fn instantiate_counter(&mut self, counter_code_id: u64) -> Addr {
        self.instantiate_contract(
            counter_code_id,
            "counter",
            &counter_base::counter::msg::InstantiateMsg {},
        )
    }

    fn migrate_counter_new(
        &mut self,
        sender: ProjectAccount,
        counter_address: Addr,
        counter_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse> {
        self.app
            .migrate_contract(
                sender.into(),
                counter_address,
                &migrate_msg,
                counter_new_code_id,
            )
            .map_err(parse_err)
    }
}
