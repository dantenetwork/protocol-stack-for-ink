import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import fs from 'fs';
import 'dotenv/config'

const provider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({provider});

const keyring = new Keyring({ type: 'sr25519' });
let data = fs.readFileSync('./.secret/keyPair.json');
const sender = keyring.addFromJson(JSON.parse(data.toString()));
sender.decodePkcs8(process.env.PASSWORD);
// console.log(sender)


const abiFile = fs.readFileSync('./abi/protocol.json');
const contract = new ContractPromise(api, JSON.parse(abiFile), process.env.CONTRACT_ADDRESS);

// const myABI = new Abi(JSON.parse(abiFile));
// // console.log(myABI.messages);
// const encodedInfo = myABI.findMessage('submit_message');
// console.log(encodedInfo);
// console.log(encodedInfo.toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}]));

const calleeFile = fs.readFileSync('./abi/callee.json');
const calleeABI = new Abi(JSON.parse(calleeFile));
const calleeEncode = calleeABI.findMessage('encode_user_multi_params').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}, "hthuang", 666]);
// const calleeEncode = calleeABI.findMessage('encode_user_defined_struct').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}]);
// console.log(calleeABI.findMessage('encode_user_defined_struct'));

console.log(calleeEncode)
// const ecdStr = Array.prototype.map.call(calleeEncode, (x) => ('00' + x.toString(16).slice(-2)));
function toHexString(byteArray) {
  return '0x' + Array.from(byteArray, function(byte) {
    return ('0' + (byte & 0xFF).toString(16)).slice(-2);
  }).join('')
}

let ecdStr = '0x';
for (let i = 1; i < calleeEncode.length; ++i){
  let stemp = calleeEncode[i].toString(16);
  if (stemp.length < 2){
    stemp = '0' + stemp;
  }
  ecdStr += stemp;
}
let newone = calleeEncode.slice(1);
let a = toHexString(newone);
console.log(ecdStr, a);

// Read from the contract via an RPC call
async function query() {
  const value = 0; // only useful on isPayable messages
  // NOTE the apps UI specified these in mega units
  const gasLimit = -1;
  
  // const storage_deposit_limit = 3n * 1000000n;
  
  // Perform the actual read (with one param, which is an user defined struct)
  // (We perform the send from an account, here using address created from a Json)
  // const { gasConsumed, result, output } = await contract.query['submitMessage'](sender.address, { value, gasLimit }, 
  //                                         {"name": "Nika", "age": 18, "phones": ["123", "456"]});

  // const calleeEncode = calleeABI.findMessage('encode_user_defined_struct').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}]);
  // 0x104e696b6112000000080c3132330c343536
  const { gasConsumed, result, output } = await contract.query['callToContracts'](sender.address, {value, gasLimit }, 
                                          "5CgMjHnZm7VAi8x9HrB4b8FXYytnUj1pqNUH92yUmY9A7g8C", ecdStr);
  
  // The actual result from RPC as `ContractExecResult`
  console.log(result.toHuman());
  
  // gas consumed
  console.log(gasConsumed.toHuman());

  // check if the call was successful
  if (result.isOk) {
    // should output 123 as per our initial set (output here is an i32)
    console.log('Success', output.toHuman());
  } else {
    console.error('Error', result.asErr);
  }
}

async function call() {
  // We will use these values for the execution
  const value = 0; // only useful on isPayable messages
  const gasLimit = -1;

  // Send the transaction, like elsewhere this is a normal extrinsic
  // with the same rules as applied in the API (As with the read example,
  // additional params, if required can follow - here only one is needed)
  await contract.tx
    .flip({ value, gasLimit })
    .signAndSend(sender, (result) => {
      console.log('result', result.isInBlock, result.isFinalized, result.isError, result.isWarning);
      if (result.status.isInBlock) {
        console.log('in a block');
        // console.log(result);
      } else if (result.status.isFinalized) {
        console.log('finalized');
      }
    });
}

query()

// call()