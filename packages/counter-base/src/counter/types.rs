use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::constants::CHAIN_ID_DEV;

#[cw_serde]
pub enum ActionType {
    // TODO: add Mul
    Add,
    Sub,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    chain_id_dev: String,
}

impl Config {
    pub fn new(admin: &Addr) -> Self {
        Self {
            admin: admin.to_owned(),
            chain_id_dev: String::from(CHAIN_ID_DEV),
        }
    }

    pub fn get_chain_id(&self) -> String {
        self.chain_id_dev.clone()
    }
}
