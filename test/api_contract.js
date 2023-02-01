
const ApiPromise = require('@polkadot/api');
const WsProvider = require('@polkadot/rpc-provider');
const options = require('@astar-network/astar-api');
//import { ApiPromise } from '@polkadot/api';
//import { WsProvider } from '@polkadot/rpc-provider';
//import { options } from '@astar-network/astar-api';

async function main() {
    //const provider = new WsProvider('ws://localhost:9944');
    // OR
    const provider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    // Use the api
    // For example:
    console.log((await api.rpc.system.properties()).toHuman());

    process.exit(0);
}

main()
