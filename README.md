### Project Description

Migratable Counter is a project to demonstrate contract migration in testing environment and on testnet.

To migrate from v1.0.0 (`contracts/counter`) to v2.0.0 (`contracts/counter-new`) following changes are required:
1. Remove `ResetCounter {}` and add `SetCounter { value: Uint128 }` in `msg.rs`
2. Update `QueryCounters {} -> Vec<(Addr, Uint128)>` to `QueryCounters { addresses: Option<Vec<String>> } -> Vec<QueryCountersResponse>` and implement `struct QueryCountersResponse { owner: Addr, counter_value: Uint128 }` in `msg.rs`
3. Add `ActionType::Mul` in `types.rs`
4. Update `TOTAL_CALLS: Item<u32>` to `TOTAL_CALLS: Item<Uint128>` in `state.rs`
5. Add `pub const TOTAL_CALLS_PREVIOUS: Item<Uint128> = Item::new("total v1.0.0 contract calls");` and save `TOTAL_CALLS` value to `TOTAL_CALLS_PREVIOUS`. `TOTAL_CALLS` must be cleared

`COUNTERS` state and contract balances must be preserved.
