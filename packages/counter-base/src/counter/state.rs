use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use crate::counter::types::Config;

pub const CONTRACT_NAME: &str = "crates.io:counter";

pub const CONFIG: Item<Config> = Item::new("config");

// TODO: replace u32 with Uint128
pub const TOTAL_CALLS: Item<u32> = Item::new("total contract calls");

pub const COUNTERS: Map<&Addr, Uint128> = Map::new("counter value by owner address");

// TODO: add pub const TOTAL_DEPOSITED: Item<Uint128> = Item::new("total coins deposited");
