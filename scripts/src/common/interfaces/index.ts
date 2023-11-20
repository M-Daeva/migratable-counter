type NetworkName = "STARGAZE";

type NetworkConfig = {
  [network in NetworkName]: {
    BASE: BaseNetworkConfig;
    CONTRACTS: ContractsConfig[];
  };
};

type BaseNetworkConfig = {
  PREFIX: string;
  DENOM: string;
  CHAIN_ID: string;
  RPC_LIST: string[];
  GAS_PRICE_AMOUNT: number;
  STORE_CODE_GAS_MULTIPLIER: number;
};

type ContractsConfig = {
  WASM: string;
  LABEL: string;
  INIT_MSG: any;
  DATA: ContractData;
};

type ContractData = {
  CODE: number;
  ADDRESS: string;
};

export type { NetworkConfig, NetworkName, ContractsConfig };

export { BaseNetworkConfig, ContractData };
