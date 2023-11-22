import { NetworkConfig } from "../../common/interfaces";
import { InstantiateMsg as CounterInstantiateMsg } from "../codegen/Counter.types";
import { InstantiateMsg as CounterNewInstantiateMsg } from "../codegen/CounterNew.types";
import STARGAZE_COUNTER from "./stargaze-counter.json";
import STARGAZE_COUNTER_NEW from "./stargaze-counter_new.json";

const COUNTER_WASM = "counter.wasm";
const COUNTER_NEW_WASM = "counter_new.wasm";

const counterInitMsg: CounterInstantiateMsg = {};
const counterNewInitMsg: CounterNewInstantiateMsg = {};

const NETWORK_CONFIG: NetworkConfig = {
  STARGAZE: {
    BASE: {
      PREFIX: "stars",
      DENOM: "ustars",
      CHAIN_ID: "elgafar-1",
      RPC_LIST: ["https://rpc.elgafar-1.stargaze-apis.com:443"],
      GAS_PRICE_AMOUNT: 0.04,
      STORE_CODE_GAS_MULTIPLIER: 20,
    },
    CONTRACTS: [
      {
        WASM: COUNTER_WASM,
        LABEL: "counter",
        INIT_MSG: counterInitMsg,
        DATA: {
          CODE: STARGAZE_COUNTER.CODE,
          ADDRESS: STARGAZE_COUNTER.ADDRESS,
        },
      },
      {
        WASM: COUNTER_NEW_WASM,
        LABEL: "counter",
        INIT_MSG: counterNewInitMsg,
        DATA: {
          CODE: STARGAZE_COUNTER_NEW.CODE,
          ADDRESS: STARGAZE_COUNTER_NEW.ADDRESS,
        },
      },
    ],
  },
};

export { NETWORK_CONFIG, COUNTER_WASM, COUNTER_NEW_WASM };
