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
// console.log(sender.address);


const abiFile = fs.readFileSync('../contracts/Protocol/target/ink/metadata.json');
const contract = new ContractPromise(api, JSON.parse(abiFile), process.env.CONTRACT_ADDRESS);

const calleeFile = fs.readFileSync('../contracts/callee/target/ink/metadata.json');
const calleeABI = new Abi(JSON.parse(calleeFile));
const calleeEncode = calleeABI.findMessage('get_payload').toU8a([{'items': [{'n': 123, 't': 'InkI128', 'v': '0x12345678'}], 'vecs': [{'n': 100123, 't': 'InkU16', 'v': ['0x99', '0x88', '0x65535']}]}]);

let ecdStr = '0x';
for (let i = 1; i < calleeEncode.length; ++i){
  let stemp = calleeEncode[i].toString(16);
  if (stemp.length < 2){
    stemp = '0' + stemp;
  }
  ecdStr += stemp;
}

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
    console.log(ecdStr);
    const { gasConsumed, result, output } = await contract.query['submitMessage'](sender.address, {value, gasLimit }, 
                                            process.env.CALLEE_CONTRACT, ecdStr);
    
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

  query()