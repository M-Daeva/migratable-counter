import { l } from "../utils";
import { toBase64, fromUtf8 } from "@cosmjs/encoding";
import { CounterMsgComposer } from "../codegen/Counter.message-composer";
import { CounterQueryClient } from "../codegen/Counter.client";
import { NETWORK_CONFIG, COUNTER_WASM } from "../config";
import {
  getCwClient,
  signAndBroadcastWrapper,
  getExecuteContractMsg,
} from "./clients";
import { ActionType } from "../codegen/Counter.types";
import {
  SigningCosmWasmClient,
  CosmWasmClient,
  MsgExecuteContractEncodeObject,
} from "@cosmjs/cosmwasm-stargate";
import {
  DirectSecp256k1HdWallet,
  OfflineSigner,
  OfflineDirectSigner,
  coin,
} from "@cosmjs/proto-signing";
import { NetworkName } from "../interfaces";

function addSingleTokenToComposerObj(
  obj: MsgExecuteContractEncodeObject,
  amount: number,
  denom: string
): MsgExecuteContractEncodeObject {
  const {
    value: { contract, sender, msg },
  } = obj;

  if (!(contract && sender && msg)) {
    throw new Error(`${msg} parameters error!`);
  }

  return getSingleTokenExecMsg(
    contract,
    sender,
    JSON.parse(fromUtf8(msg)),
    amount,
    denom
  );
}

function getSingleTokenExecMsg(
  contractAddress: string,
  senderAddress: string,
  msg: any,
  amount?: number,
  denom?: string
) {
  // get msg without funds
  if (!(denom && amount)) {
    return getExecuteContractMsg(contractAddress, senderAddress, msg, []);
  }

  // get msg with native token
  return getExecuteContractMsg(contractAddress, senderAddress, msg, [
    coin(amount, denom),
  ]);
}

async function getCwExecHelpers(
  network: NetworkName,
  rpc: string,
  owner: string,
  signer: (OfflineSigner & OfflineDirectSigner) | DirectSecp256k1HdWallet
) {
  const { CONTRACTS } = NETWORK_CONFIG[network];

  const COUNTER_CONTRACT = CONTRACTS.find((x) => x.WASM === COUNTER_WASM);
  if (!COUNTER_CONTRACT) throw new Error("COUNTER_CONTRACT in not found!");

  const cwClient = await getCwClient(rpc, owner, signer);
  if (!cwClient) throw new Error("cwClient is not found!");

  const signingClient = cwClient.client as SigningCosmWasmClient;
  const _signAndBroadcast = signAndBroadcastWrapper(signingClient, owner);

  const counterMsgComposer = new CounterMsgComposer(
    owner,
    COUNTER_CONTRACT.DATA.ADDRESS
  );

  async function _msgWrapperWithGasPrice(
    msgs: MsgExecuteContractEncodeObject[],
    gasPrice: string
  ) {
    const tx = await _signAndBroadcast(msgs, gasPrice);
    l("\n", tx, "\n");
    return tx;
  }

  async function cwCreateCounter(
    paymentAmount: number,
    paymentDenom: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        addSingleTokenToComposerObj(
          counterMsgComposer.createCounter(),
          paymentAmount,
          paymentDenom
        ),
      ],
      gasPrice
    );
  }

  async function cwUpdateCounter(
    actionType: ActionType,
    value: number,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [counterMsgComposer.updateCounter({ actionType, value: `${value}` })],
      gasPrice
    );
  }

  async function cwResetCounter(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [counterMsgComposer.resetCounter()],
      gasPrice
    );
  }

  return {
    cwCreateCounter,
    cwUpdateCounter,
    cwResetCounter,
  };
}

async function getCwQueryHelpers(network: NetworkName, rpc: string) {
  const { CONTRACTS } = NETWORK_CONFIG[network];

  const COUNTER_CONTRACT = CONTRACTS.find((x) => x.WASM === COUNTER_WASM);
  if (!COUNTER_CONTRACT) throw new Error("COUNTER_CONTRACT in not found!");

  const cwClient = await getCwClient(rpc);
  if (!cwClient) throw new Error("cwClient is not found!");

  const cosmwasmQueryClient: CosmWasmClient = cwClient.client;

  const counterQueryClient = new CounterQueryClient(
    cosmwasmQueryClient,
    COUNTER_CONTRACT.DATA.ADDRESS
  );

  async function cwQueryCounters() {
    const res = await counterQueryClient.queryCounters();
    l("\n", res, "\n");
    return res;
  }

  async function cwQueryConfig() {
    const res = await counterQueryClient.queryConfig();
    l("\n", res, "\n");
    return res;
  }

  return {
    cwQueryCounters,
    cwQueryConfig,
  };
}

export { getCwExecHelpers, getCwQueryHelpers };
