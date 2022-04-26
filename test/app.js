import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import fs from 'fs';

const wsProvider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create();
const res = await api.query.contracts.contractInfoOf("5HkV3zDUqdCe4qT1Z8DhkNwGX5hKen9cCbGBu6U7uGVMZ7t1");
// console.log(await api.query.system.number());
// console.log(await api.query.contracts.nonce());
// console.log(res.registry);


const abiFile = fs.readFileSync('./abi/metadata.json');
const abi = new Abi(JSON.parse(abiFile), api.registry.getChainProperties());
// const contract = new ContractPromise(api, JSON.parse(abiFile), "5HkV3zDUqdCe4qT1Z8DhkNwGX5hKen9cCbGBu6U7uGVMZ7t1");
// console.log(contract);

// const now = await api.query.timestamp.now();
// const { nonce, data: balance } = await api.query.system.account("5HkV3zDUqdCe4qT1Z8DhkNwGX5hKen9cCbGBu6U7uGVMZ7t1");
// console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);

// Read from the contract via an RPC call
const value = 0; // only useful on isPayable messages
// NOTE the apps UI specified these in mega units
const gasLimit = 3000n * 1000000n;

const storage_deposit_limit = 3000n * 1000000n;

// const callValue = await contract.query.get({ value, gasLimit } );

// console.log(contract.query.get({gasLimit, storage_deposit_limit, value}));

// const callValue = await contract
//   .exec('get');
console.log(abi.findMessage('get'));

const callValue = await api.tx.contracts
  .call("5H5irMCWNwcsZy3pXua9bpbSrmDq4oUGdShzQ2q7LJoPSPE4", value, gasLimit, storage_deposit_limit, abi.findMessage('get'))
  .signAndSend();

console.log(callValue);
