import { NetworkConfig } from "../../common/interfaces";
import { InstantiateMsg } from "../codegen/Counter.types";
import STARGAZE_COUNTER from "./stargaze-counter.json";

const COUNTER_WASM = "counter.wasm";

const counterInitMsg: InstantiateMsg = {};

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
        LABEL: "counter-dev-1.0",
        INIT_MSG: counterInitMsg,
        DATA: {
          CODE: STARGAZE_COUNTER.CODE,
          ADDRESS: STARGAZE_COUNTER.ADDRESS,
        },
      },
    ],
  },
};

export { NETWORK_CONFIG, COUNTER_WASM };
