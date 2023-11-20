### Project Description

Migratable Counter is a project to learn migrating contracts on testnet and in testing environment.

To migrate from v1 to v2 following changes are required:
1. Remove `ResetCounter {}` and add `SetCounter { value: Uint128 }` in `msg.rs`
2. Update `QueryCounters {} -> Vec<(Addr, Uint128)>` to `QueryCounters { addresses: Option<Vec<String>> } -> Vec<QueryCountersResponse>` and implement `struct QueryCountersResponse { owner: Addr, counter_value: Uint128 }` in `msg.rs`
3. Add `QueryTotalDeposited {} -> Uint128` in `msg.rs`
4. Replace `TOTAL_CALLS: Item<u32>` with `TOTAL_CALLS: Item<Uint128>` in `state.rs`
5. Add `pub const TOTAL_DEPOSITED: Item<Uint128> = Item::new("total coins deposited");` in `state.rs`
6. Add `ActionType::Mul` in `types.rs`
7. Add `Reply` entry point to make possible creating counter on `UpdateCounter` or `SetCounter` if it wasn't created before

`COUNTERS` state and contract balances must be preserved.
