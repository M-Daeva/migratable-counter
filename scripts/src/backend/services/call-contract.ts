import { getGasPriceFromChainRegistryItem } from "../../common/account/clients";
import { getSigner } from "../account/signer";
import { getSeed } from "./get-seed";
import { l } from "../../common/utils";
import { chains } from "chain-registry";
import { NetworkName } from "../../common/interfaces";
import { readFile } from "fs/promises";
import { PATH } from "../envs";
import { NETWORK_CONFIG, COUNTER_WASM } from "../../common/config";
import { getSgQueryHelpers } from "../../common/account/sg-helpers";
import {
  getCwExecHelpers,
  getCwQueryHelpers,
} from "../../common/account/cw-helpers";

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

    const testWallets: {
      SEED_DAPP: string;
    } = JSON.parse(await readFile(PATH.TO_TEST_WALLETS, { encoding: "utf8" }));

    const seed = await getSeed(testWallets.SEED_DAPP);
    if (!seed) throw new Error("Seed is not found!");

    const chain = chains.find((x) => x.chain_id == CHAIN_ID);
    if (!chain) throw new Error(`${CHAIN_ID} config is not found!`);

    const gasPrice = getGasPriceFromChainRegistryItem(chain);

    const { signer, owner } = await getSigner(PREFIX, seed);

    const { getAllBalances } = await getSgQueryHelpers(RPC);

    const { cwQueryConfig, cwQueryCounters } = await getCwQueryHelpers(
      network,
      RPC
    );

    const { cwCreateCounter, cwResetCounter, cwUpdateCounter } =
      await getCwExecHelpers(network, RPC, owner, signer);

    // await cwCreateCounter(1, DENOM, gasPrice);

    await cwUpdateCounter("add", 5, gasPrice);

    await cwQueryCounters();
  } catch (error) {
    l(error);
  }
}

main("STARGAZE");
