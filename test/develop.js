import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import fs from 'fs';
import 'dotenv/config'
import { bool, _void, str, u8, u16, u32, u64, u128, Enum, Struct, Vector, Option, Bytes } from "scale-ts"

let MsgType = Enum({
  InkString: _void,
  InkU8: _void,
  InkU16: _void,
  InkU32: _void,
  InkU64: _void,
  InkU128: _void,
  InkI8: _void,
  InkI16: _void,
  InkI32: _void,
  InkI64: _void,
  InkI128: _void,
  InkStringArray: _void,
  UserData: _void,
});

let PayloadItem = Struct({
  n: u128,
  t: MsgType,
  v: Vector(u8)
});

let PayloadVec = Struct({
  n: u128,
  t: MsgType,
  v: Vector(Vector(u8))
});

let PayloadMessage = Struct({
  items: Option(Vector(PayloadItem)),
  vecs: Option(Vector(PayloadVec))
})

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
  const { gasConsumed, result, output } = await crossChainContract.query['crossChainBase::getReceivedMessage'](sender.address, {value, gasLimit }, 
                                          "ETHEREUM", 1);
  
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

async function test_message() {
  const value = 0; // only useful on isPayable messages
  // NOTE the apps UI specified these in mega units
  const gasLimit = -1;
  
  // const storage_deposit_limit = 3n * 1000000n;
  
  // Perform the actual read (with one param, which is an user defined struct)
  // (We perform the send from an account, here using address created from a Json)
  // const { gasConsumed, result, output } = await contract.query['submitMessage'](sender.address, { value, gasLimit }, 
  //                                         {"name": "Nika", "age": 18, "phones": ["123", "456"]});

  let payload = await test_scale_codec1();
  
  let revert = PayloadMessage.dec(payload);
  console.log('revert', JSON.stringify(revert.items[0].t));
  console.log('revert', toHexString(revert.items[0].v));
  let a = Vector(str).dec(toHexString(revert.items[0].v));
  console.log('a', a);
}

async function pushMessage() {
  // We will use these values for the execution
  const value = 0; // only useful on isPayable messages
  const gasLimit = -1;

  let payload = '0x0104010000000000000000000000000000000bd81020504f4c4b41444f54244772656574696e6773584772656574696e672066726f6d20504f4c4b41444f5428323032322d30362d303100';

  let message = {
    id: '1',
    from_chain: 'ETHEREUM',
    sender: '0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b',
    signer: '0x3aE841B899Ae4652784EA734cc61F524c36325d1',
    sqos: {
      reveal: '1'
    },
    contract: '5DeiQFwpYh7cJ5Rx5pnHgHHWPBbgq4qkyf3Q8G9CE6ZvEFLu',
    action: '0x3a6e9696',
    data: payload,
    session: {
      msg_type: '0',
      id: '0'
    },
    executed: false,
    error_code: 0
  }
  console.log(message);

  // Send the transaction, like elsewhere this is a normal extrinsic
  // with the same rules as applied in the API (As with the read example,
  // additional params, if required can follow - here only one is needed)
  await crossChainContract.tx
    ['crossChainBase::receiveMessage']({ value, gasLimit }, message)
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

async function test_scale_codec1() {
  let data = [ 'POLKADOT', 'Greetings', 'Greeting from POLKADOT', '2022-06-01' ];

  let payload = {
    items:[]
  };

  let item = {};
  item.n = BigInt(1);
  item.t = {tag: 'InkStringArray'};
  item.v = Array.from(Vector(str).enc(data));
  payload.items.push(item);
  
  console.log(toHexString(PayloadMessage.enc(payload)));
  return toHexString(PayloadMessage.enc(payload));
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

    let item1 = {
      n: 1n,
      t: {tag: 'InkU32'},
      v: Array.from(enc_param1)
    }

    let item2 = {
      n: 2n,
      t: {tag: 'InkString'},
      v: Array.from(enc_param2)
    }

    let item3 = {
      n: 3n,
      t: {tag: 'UserData'},
      v: Array.from(enc_param3)
    }

    let payload = {
      items: [item1, item2, item3]
    };

    console.log('payload', payload);
    console.log(toHexString(PayloadItem.enc(item1)));
    console.log(toHexString(PayloadItem.enc(item2)));
    console.log(toHexString(PayloadItem.enc(item3)));
    console.log(toHexString(PayloadMessage.enc(payload)));
    return toHexString(PayloadMessage.enc(payload));
}
// 0x010c0100000000000000000000000000000003109a0200000200000000000000000000000000000000201c68746875616e67030000000000000000000000000000000b501867656f72676521000000080c3132330c34353600
// 0x010c0100000000000000000000000000000003109a0200000200000000000000000000000000000000201c68746875616e67030000000000000000000000000000000b501867656f72676521000000080c3132330c34353600
// test_scale_codec()
// test_scale_codec1()
query()
// test_message()
function test() {
  let api = require("@polkadot/api");
  let api_contract = require("@polkadot/api-contract");
  console.log(api, api_contract);
}

// test()

// pushMessage()