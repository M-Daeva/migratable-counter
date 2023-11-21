use cosmwasm_std::{Addr, MessageInfo, Uint128};
use cw_storage_plus::{Deque, Item, Map};

use crate::counter_new::types::ActionType;

pub const SET_COUNTER_REPLY_ID: u64 = 1;
pub const UPDATE_COUNTER_REPLY_ID: u64 = 2;

pub const CONTRACT_NAME: &str = "crates.io:counter";

pub const TOTAL_CALLS: Item<Uint128> = Item::new("total contract calls");
pub const TOTAL_CALLS_PREVIOUS: Item<Uint128> = Item::new("total v1.0.0 contract calls");
pub const TOTAL_DEPOSITED: Item<Uint128> = Item::new("total coins deposited");

pub const COUNTERS: Map<&Addr, Uint128> = Map::new("counter value by owner address");

pub const UPDATE_COUNTER_QUEUE: Deque<(MessageInfo, ActionType, Uint128)> =
    Deque::new("update counter args");
pub const SET_COUNTER_QUEUE: Deque<(MessageInfo, Uint128)> = Deque::new("set counter args");
