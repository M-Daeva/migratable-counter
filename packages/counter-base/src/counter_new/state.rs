use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const CONTRACT_NAME: &str = "crates.io:counter";

pub const TOTAL_CALLS: Item<Uint128> = Item::new("total contract calls");
pub const TOTAL_CALLS_PREVIOUS: Item<Uint128> = Item::new("total v1.0.0 contract calls");
pub const TOTAL_DEPOSITED: Item<Uint128> = Item::new("total coins deposited");

pub const COUNTERS: Map<&Addr, Uint128> = Map::new("counter value by owner address");
