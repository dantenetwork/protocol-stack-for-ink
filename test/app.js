import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi } from '@polkadot/api-contract';

const wsProvider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create();
const res = await api.query.contracts.contractInfoOf("5CNcbf4FDSgzJ7pBMpe1gxRkndXEK9thRtCjb1fHFbhRwK57");
// console.log(await api.query.system.number());
// console.log(await api.query.contracts.nonce());
console.log(res.registry);

const abiFile = fs.readFileSync('./abi/metadata.json');
const abi = new Abi(JSON.parse(abiFile), api.registry.getChainProperties());
console.log(abi);
const contract = new ContractPromise(api, abi, "5H5irMCWNwcsZy3pXua9bpbSrmDq4oUGdShzQ2q7LJoPSPE4");


// Read from the contract via an RPC call
const value = 0; // only useful on isPayable messages
// NOTE the apps UI specified these in mega units
const gasLimit = 3000n * 1000000n;

// const callValue = await contract.query.get({ value, gasLimit } );

const callValue = await contract
  .read('get', { value, gasLimit })
  .send("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
// const callValue = await api.tx.contracts
//   .call("5H5irMCWNwcsZy3pXua9bpbSrmDq4oUGdShzQ2q7LJoPSPE4", ivalue, igasLimit, abi.get())
//   .send("5H5irMCWNwcsZy3pXua9bpbSrmDq4oUGdShzQ2q7LJoPSPE4");

console.log(callValue);
