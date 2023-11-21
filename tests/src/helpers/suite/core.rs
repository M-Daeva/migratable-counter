use cosmwasm_std::{coin, Addr, Coin, Timestamp};
use cw_multi_test::{App, AppResponse, Executor};

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::helpers::suite::{
    codes::WithCodes,
    types::{GetDecimals, ProjectAccount, ProjectCoin, WrappedResponse},
};

pub struct Project {
    pub app: App,
    pub logs: WrappedResponse,
    contract_counter: u16,

    // contract code id
    counter_code_id: u64,
    counter_new_code_id: u64,

    // contract address
    counter_address: Addr,
}

impl Project {
    pub fn create_project_with_balances() -> Self {
        Self {
            app: Self::create_app_with_balances(),
            logs: WrappedResponse::Execute(Ok(AppResponse::default())),
            contract_counter: 0,

            counter_code_id: 0,
            counter_new_code_id: 0,

            counter_address: Addr::unchecked(""),
        }
    }

    pub fn new(chain_id_mocked: Option<&str>) -> Self {
        // create app and distribute coins to accounts
        let mut project = Self::create_project_with_balances();

        // set specific chain_id to prevent execution of mocked actions on real networks
        let chain_id = chain_id_mocked.unwrap_or(counter_base::constants::CHAIN_ID_DEV);
        project
            .app
            .update_block(|block| block.chain_id = String::from(chain_id));

        // register contracts code

        // contracts
        let counter_code_id = project.store_counter_code();
        let counter_new_code_id = project.store_counter_new_code();

        let counter_address = project.instantiate_counter(counter_code_id);

        project = Self {
            counter_code_id,
            counter_new_code_id,
            counter_address,
            ..project
        };

        project
    }

    // code id getters
    pub fn get_counter_code_id(&self) -> u64 {
        self.counter_code_id
    }

    pub fn get_counter_new_code_id(&self) -> u64 {
        self.counter_new_code_id
    }

    // contract address getters
    pub fn get_counter_address(&self) -> Addr {
        self.counter_address.clone()
    }

    // utils
    pub fn increase_contract_counter(&mut self, step: u16) {
        self.contract_counter += step;
    }

    pub fn get_last_contract_address(&self) -> String {
        format!("contract{}", self.contract_counter)
    }

    pub fn get_timestamp(&self) -> Timestamp {
        self.app.block_info().time
    }

    pub fn wait(&mut self, delay_ns: u64) {
        self.app.update_block(|block| {
            block.time = block.time.plus_nanos(delay_ns);
            block.height += delay_ns / 5_000_000_000;
        });
    }

    pub fn instantiate_contract(
        &mut self,
        code_id: u64,
        label: &str,
        init_msg: &impl Serialize,
    ) -> Addr {
        self.increase_contract_counter(1);

        self.app
            .instantiate_contract(
                code_id,
                ProjectAccount::Admin.into(),
                init_msg,
                &[],
                label,
                Some(ProjectAccount::Admin.to_string()),
            )
            .unwrap()
    }

    fn create_app_with_balances() -> App {
        App::new(|router, _api, storage| {
            for project_account in ProjectAccount::iter() {
                let funds: Vec<Coin> = ProjectCoin::iter()
                    .map(|project_coin| {
                        let amount = project_account.get_initial_funds_amount()
                            * 10u128.pow(project_coin.get_decimals() as u32);

                        coin(amount, project_coin.to_string())
                    })
                    .collect();

                router
                    .bank
                    .init_balance(storage, &project_account.into(), funds)
                    .unwrap();
            }
        })
    }
}

pub fn assert_error<S: std::fmt::Debug + ToString>(subject: &S, err: impl ToString + Sized) {
    speculoos::assert_that(subject).matches(|x| x.to_string().contains(&err.to_string()));
}
