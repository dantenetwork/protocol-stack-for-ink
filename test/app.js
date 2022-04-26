import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi } from '@polkadot/api-contract';

const wsProvider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create();
const res = await api.query.contracts.contractInfoOf("5CNcbf4FDSgzJ7pBMpe1gxRkndXEK9thRtCjb1fHFbhRwK57");
// console.log(await api.query.system.number());
// console.log(await api.query.contracts.nonce());
console.log(res.registry);

// const keyring = new Keyring({ type: 'sr25519' });

// // (Advanced, development-only) add with an implied dev seed and hard derivation
// const nika = keyring.addFromUri('//Nika', { name: 'Nika default' });

// const nikaInfo = await api.query.system.account(nika.address);
// console.log(`${nika.meta}, ${nikaInfo.data.free.toString(10)}`);

// console.log(keyring.pairs);
