import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import fs from 'fs';
import 'dotenv/config'
import { bool, _void, str, u32, Enum, Struct, Vector } from "scale-ts"
import textEncoding from 'text-encoding';

// network
const provider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({provider});

// key
const keyring = new Keyring({ type: 'sr25519' });
let data = fs.readFileSync('./.secret/keyPair.json');
const sender = keyring.addFromJson(JSON.parse(data.toString()));
sender.decodePkcs8(process.env.PASSWORD);

// cross-chain contract
const crossChainABIRaw = fs.readFileSync('./abi/cross_chain.json');
const crossChainContract = new ContractPromise(api, JSON.parse(crossChainABIRaw), process.env.CONTRACT_ADDRESS);

// locker-mock contract
const calleeAbiRaw = fs.readFileSync('./abi/callee.json');
const calleeABI = new Abi(JSON.parse(calleeAbiRaw));
// const calleeEncode = calleeABI.findMessage('encode_user_multi_params').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}, "hthuang", 666]);
// const calleeDecode = calleeABI.findMessage('encode_user_multi_params').fromU8a(calleeEncode.subarray(5));

// const calleeJson = JSON.parse(calleeAbi);
// let json = {V3: {spec: {messages: []}}};
// json.V3.spec.messages.push(calleeJson.V3.spec.messages[3]);
// json.V3.types = calleeJson.V3.types;
// const calleeABI2 = new Abi(json);
// const calleeEncode2 = calleeABI2.findMessage('encode_user_multi_params').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}, "hthuang", 666]);

// const ecdStr = Array.prototype.map.call(calleeEncode, (x) => ('00' + x.toString(16).slice(-2)));

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
  console.log(ecdStr);
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

async function pushMessage() {
  // We will use these values for the execution
  const value = 0; // only useful on isPayable messages
  const gasLimit = -1;

  const param1 = 666;
  const param2 = 'hthuang';
  const param3 = {'items': [{'n': 1, 't': 'InkI32', 'v': '0x12345678'}], 'vecs': [{'n': 100123, 't': 'InkU16', 'v': ['0x99', '0x88', '0x65535']}]};

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

function toHexString(byteArray) {
    return '0x' + Array.from(byteArray, function(byte) {
        return ('0' + (byte & 0xFF).toString(16)).slice(-2);
    }).join('')
}

async function test_scale_codec() {
    let enc_param1 = u32.enc(666);

    let enc_param2 = str.enc('hthuang');

    let MessageDetail = Struct({
        name: str,
        age: u32,
        phones: Vector(str)
    })
    let enc_param3 = MessageDetail.enc({
        name: 'george',
        age: 33,
        phones: ['123', '456']
    });

    console.log(enc_param1, enc_param2, enc_param3);
    console.log(toHexString(enc_param1));
    console.log(toHexString(enc_param2));
    console.log(toHexString(enc_param3));
}
// 0x010c0100000000000000000000000000000003109a0200000200000000000000000000000000000000109a020000030000000000000000000000000000000b501867656f72676521000000080c3132330c34353600
test_scale_codec()

// query()

// call()