import { getSigner } from "../account/signer";
import { getSeed } from "./get-seed";
import { l } from "../../common/utils";
import { toUtf8 } from "@cosmjs/encoding";
import { chains } from "chain-registry";
import { NetworkName } from "../../common/interfaces";
import { readFile } from "fs/promises";
import { PATH } from "../envs";
import { getSgQueryHelpers } from "../../common/account/sg-helpers";
import { MigrateMsg } from "../../common/codegen/CounterNew.types";
import { calculateFee } from "@cosmjs/stargate";
import { MsgMigrateContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import {
  SigningCosmWasmClient,
  MsgMigrateContractEncodeObject,
} from "@cosmjs/cosmwasm-stargate";
import {
  getGasPriceFromChainRegistryItem,
  getCwClient,
} from "../../common/account/clients";
import {
  getCwExecHelpers,
  getCwQueryHelpers,
} from "../../common/account/cw-helpers";
import {
  getCwExecHelpers as getCwExecHelpersNew,
  getCwQueryHelpers as getCwQueryHelpersNew,
} from "../../common/account/cw-helpers-new";
import {
  NETWORK_CONFIG,
  COUNTER_WASM,
  COUNTER_NEW_WASM,
} from "../../common/config";

async function main(network: NetworkName) {
  try {
    const {
      BASE: {
        DENOM,
        CHAIN_ID,
        RPC_LIST: [RPC],
        PREFIX,
      },
      CONTRACTS,
    } = NETWORK_CONFIG[network];

    const COUNTER_CONTRACT = CONTRACTS.find((x) => x.WASM === COUNTER_WASM);
    if (!COUNTER_CONTRACT) throw new Error("COUNTER_CONTRACT in not found!");

    const COUNTER_NEW_CONTRACT = CONTRACTS.find(
      (x) => x.WASM === COUNTER_NEW_WASM
    );
    if (!COUNTER_NEW_CONTRACT) {
      throw new Error("COUNTER_NEW_CONTRACT in not found!");
    }

    const testWallets: {
      SEED_DAPP: string;
    } = JSON.parse(await readFile(PATH.TO_TEST_WALLETS, { encoding: "utf8" }));

    const seed = await getSeed(testWallets.SEED_DAPP);
    if (!seed) throw new Error("Seed is not found!");

    const chain = chains.find((x) => x.chain_id == CHAIN_ID);
    if (!chain) throw new Error(`${CHAIN_ID} config is not found!`);

    const gasPrice = getGasPriceFromChainRegistryItem(chain);

    const { signer, owner } = await getSigner(PREFIX, seed);
    const cwClient = await getCwClient(RPC, owner, signer);
    if (!cwClient) throw new Error("cwClient is not found!");

    const signingClient = cwClient.client as SigningCosmWasmClient;

    const { getAllBalances } = await getSgQueryHelpers(RPC);

    const counterQueries = await getCwQueryHelpers(network, RPC);
    const counterExecutes = await getCwExecHelpers(network, RPC, owner, signer);
    const counterNewQueries = await getCwQueryHelpersNew(network, RPC);
    const counterNewExecutes = await getCwExecHelpersNew(
      network,
      RPC,
      owner,
      signer
    );

    // https://testnet.ping.pub/stargaze/tx/0E0C535CE62890BB54642649CC286F53D415AD2C7625D4ABA1BF6FEDF0342640
    await counterExecutes.cwCreateCounter(10, DENOM, gasPrice);
    // https://testnet.ping.pub/stargaze/tx/08ECFC29F68B01256C8FA8A3DDA668B2C2601A10AE6D4EED4303A8F728B775B9
    await counterExecutes.cwUpdateCounter("add", 5, gasPrice);
    await counterQueries.cwQueryCounters();
    await getAllBalances(COUNTER_CONTRACT.DATA.ADDRESS);

    const migrateMsg: MigrateMsg = "v2_0_0";

    const msg: MsgMigrateContractEncodeObject = {
      typeUrl: "/cosmwasm.wasm.v1.MsgMigrateContract",
      value: MsgMigrateContract.fromPartial({
        sender: owner,
        contract: COUNTER_CONTRACT.DATA.ADDRESS,
        codeId: COUNTER_NEW_CONTRACT.DATA.CODE,
        msg: toUtf8(JSON.stringify(migrateMsg)),
      }),
    };

    const gasSimulated = await signingClient.simulate(owner, [msg], "");
    const gasWantedSim = Math.ceil(1.2 * gasSimulated);

    // https://testnet.ping.pub/stargaze/tx/E062FCE4B3E1244338D0E3F724B8B677EC97CFB1D54ADAFFF8F84447C5971B19
    const tx = await signingClient.migrate(
      owner,
      COUNTER_CONTRACT.DATA.ADDRESS,
      COUNTER_NEW_CONTRACT.DATA.CODE,
      migrateMsg,
      calculateFee(gasWantedSim, gasPrice)
    );

    l(tx);

    // https://testnet.ping.pub/stargaze/tx/161336FB402A2234FF656C947CC42E4BA4412676A6C0286BBB7DCAC5D1F955A5
    await counterNewExecutes.cwSetCounter(42, gasPrice);
    await counterNewQueries.cwQueryCounters();
    await counterNewQueries.cwQueryTotalCallsPrevious();
    await counterNewQueries.cwQueryTotalCalls();
    await getAllBalances(COUNTER_CONTRACT.DATA.ADDRESS);
  } catch (error) {
    l(error);
  }
}

main("STARGAZE");
