/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.3.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {}
export type ExecuteMsg = {
  create_counter: {};
} | {
  update_counter: {
    action_type: ActionType;
    value: Uint128;
  };
} | {
  reset_counter: {};
};
export type ActionType = "add" | "sub";
export type Uint128 = string;
export type QueryMsg = {
  query_counters: {};
} | {
  query_config: {};
};
export type MigrateMsg = string;
export type Addr = string;
export interface Config {
  admin: Addr;
  chain_id_dev: string;
}
export type ArrayOfTupleOfAddrAndUint128 = [Addr, Uint128][];