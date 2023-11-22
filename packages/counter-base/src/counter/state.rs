use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const CONTRACT_NAME: &str = "crates.io:counter";

// TODO: 4. Replace u32 with Uint128
pub const TOTAL_CALLS: Item<u32> = Item::new("total contract calls");

pub const COUNTERS: Map<&Addr, Uint128> = Map::new("counter value by owner address");

// TODO: 5. Add pub const TOTAL_CALLS_PREVIOUS: Item<Uint128> = Item::new("total v1.0.0 contract calls");
// and save TOTAL_CALLS value to TOTAL_CALLS_PREVIOUS, TOTAL_CALLS must be cleared
