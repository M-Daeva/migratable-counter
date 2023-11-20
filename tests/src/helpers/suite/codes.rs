use cosmwasm_std::Addr;
use cw_multi_test::ContractWrapper;

use crate::helpers::suite::core::Project;

pub trait WithCodes {
    // store contracts
    fn store_counter_code(&mut self) -> u64;

    // instantiate contracts
    fn instantiate_counter(&mut self, counter_code_id: u64) -> Addr;
}

impl WithCodes for Project {
    fn store_counter_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                counter::contract::execute,
                counter::contract::instantiate,
                counter::contract::query,
            ), // .with_reply(counter::contract::reply),
        ))
    }

    fn instantiate_counter(&mut self, counter_code_id: u64) -> Addr {
        self.instantiate_contract(
            counter_code_id,
            "counter",
            &counter_base::counter::msg::InstantiateMsg {},
        )
    }
}
