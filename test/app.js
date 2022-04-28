import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import fs from 'fs';
import { map } from 'rxjs';

const wsProvider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create();
const res = await api.query.contracts.contractInfoOf("5GyubsE5CXAmsgzrjVGGUzdNhbL1H4MKqmm6YP5ka7rqjivb");
// console.log(await api.query.system.number());
// console.log(await api.query.contracts.nonce());
console.log(res.registry);


const abiFile = fs.readFileSync('./abi/metadata.json');
const abi = new Abi(JSON.parse(abiFile), api.registry.getChainProperties());
const contract = new ContractPromise(api, JSON.parse(abiFile), "5E1Hksz3SAnMsu75TBzMkrdS1QB1mqF6sbdymTFsyH65S3SY");

// const now = await api.query.timestamp.now();
// const { nonce, data: balance } = await api.query.system.account("5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV");
// console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);

// Read from the contract via an RPC call
const value = 0; // only useful on isPayable messages
// NOTE the apps UI specified these in mega units
const gasLimit = 3n * 1000000n;

const storage_deposit_limit = 3n * 1000000n;

// console.log(contract.query);
// console.log(contract.tx);

// Constuct the keying after the API (crypto has an async init)
const keyring = new Keyring({ type: 'sr25519' });

// Add Alice to our keyring with a hard-deived path (empty phrase, so uses dev)
let data = fs.readFileSync('./.secret/keyPair.json');
const sender = keyring.addFromJson(JSON.parse(data.toString()));
sender.decodePkcs8("NewNika123456");

console.log(sender);

const callValue = await contract.query['get'](sender.address, {gasLimit, value});

console.log(callValue);

// const callValue = await contract.query.get({ value, gasLimit } );

// console.log(contract.query().get({gasLimit, storage_deposit_limit, value}));

// const callValue = await contract
//   .exec('get');
// console.log(abi.findMessage('get'));

// const callValue = await api.tx.contracts
//   .query("5E1Hksz3SAnMsu75TBzMkrdS1QB1mqF6sbdymTFsyH65S3SY", value, gasLimit, storage_deposit_limit, abi.findMessage('get').toU8a([]))
//   .send(sender.address);

// console.log(callValue.toHuman());
