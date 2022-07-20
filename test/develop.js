import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { decodeAddress, encodeAddress } from '@polkadot/keyring';
import fs from 'fs';
import 'dotenv/config'
import { bool, _void, str,i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, Enum, Struct, Vector, Option, Bytes } from "scale-ts"

const InkAddressData = Struct({
  ink_address: Option(Vector(u8, 32)),
  general_address: Option(str),
  address_type: u8,
});

const MsgDetail = Enum({
  InkString: str,
  InkU8: u8,
  InkU16: u16,
  InkU32: u32,
  InkU64: u64,
  InkU128: u128,
  InkI8: i8,
  InkI16: i16,
  InkI32: i32,
  InkI64: i64,
  InkI128: i128,
  InkStringArray: Vector(str),
  InkU8Array: Vector(u8),
  InkU16Array: Vector(u16),
  InkU32Array: Vector(u32),
  InkU64Array: Vector(u64),
  InkU128Array: Vector(u128),
  InkI8Array: Vector(i8),
  InkI16Array: Vector(i16),
  InkI32Array: Vector(i32),
  InkI64Array: Vector(i64),
  InkI128Array: Vector(i128),
  InkAddress: InkAddressData,
  // UserData: Bytes,
});

const MessageItem = Struct({
  n: str,
  tv: MsgDetail
});

let MessagePayload = Struct({
  items: Option(Vector(MessageItem))
});

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

// locker contract
const lockerABIRaw = fs.readFileSync('./abi/locker.json');
const lockerContract = new ContractPromise(api, JSON.parse(lockerABIRaw), process.env.LOCKER_ADDRESS);
// const crossChainABI = new Abi(JSON.parse(crossChainABIRaw));
// let m = crossChainABI.findMessage('crossChainBase::executeMessage').toU8a(["A", 1]);
// console.log('m', toHexString(m));

// locker-mock contract
// const calleeAbiRaw = fs.readFileSync('./abi/callee.json');
// const calleeABI = new Abi(JSON.parse(calleeAbiRaw));
// console.log('calleeABI', calleeABI.findMessage)
// const calleeContract = new ContractPromise(api, JSON.parse(calleeAbiRaw), process.env.CALLEE_ADDRESS);
// const calleeEncode = calleeABI.findMessage('encode_user_multi_params').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}, "hthuang", 666]);
// // const calleeDecode = calleeABI.findMessage('encode_user_multi_params').fromU8a(calleeEncode.subarray(5));
// console.log('calleeEncode', calleeEncode)

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
  const { gasConsumed, result, output } = await lockerContract.query['receiveToken'](sender.address, {value, gasLimit },
    await test_message());
  
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

  // let payload = await test_scale_codec1();
  let payload = '0x010808746f1601d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d00020c6e756d0501000000000000000000000000000000';
  
  let revert = MessagePayload.dec(payload);
  console.log('revert', revert, revert.items[0].tv, revert.items[1].tv);
  return revert;
}

async function pushMessage(i) {
  // We will use these values for the execution
  const value = 0; // only useful on isPayable messages
  const gasLimit = -1;

  let payload = '0x0104010000000000000000000000000000000bd81020504f4c4b41444f54244772656574696e6773584772656574696e672066726f6d20504f4c4b41444f5428323032322d30362d303100';

  let message = {
    id: i,
    from_chain: 'ETHEREUM',
    sender: '0xa6666D8299333391B2F5ae337b7c6A82fa51Bc9b',
    signer: '0x3aE841B899Ae4652784EA734cc61F524c36325d1',
    sqos: [],
    contract: '0x4a0baeef90dd3c88da26e70f3121b71d037ba4b994cfa9d7d7fd900ca450738b',
    action: '0x3a6e9696',
    data: payload,
    session: {
      callback: '0x',
      id: '0'
    }
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

async function transferToken() {
  // We will use these values for the execution
  const value = 0; // only useful on isPayable messages
  const gasLimit = -1;

  // Send the transaction, like elsewhere this is a normal extrinsic
  // with the same rules as applied in the API (As with the read example,
  // additional params, if required can follow - here only one is needed)
  await lockerContract.tx
    ['transferToken']({ value, gasLimit }, 'POLKADOT', 
    {ink_address: '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d', general_address: null, address_type: 2}, 1)
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

async function sendGreeting() {
  // We will use these values for the execution
  const value = 0; // only useful on isPayable messages
  const gasLimit = -1;

  // Send the transaction, like elsewhere this is a normal extrinsic
  // with the same rules as applied in the API (As with the read example,
  // additional params, if required can follow - here only one is needed)
  function getCurrentDate() {
    var today = new Date();
    var dd = String(today.getDate()).padStart(2, '0');
    var mm = String(today.getMonth() + 1).padStart(2, '0');
    var yyyy = today.getFullYear();
    return yyyy + '-' + mm + '-' + dd;
  }

  await calleeContract.tx
    ['sendGreeting']({ value, gasLimit }, 'PLATONEVMDEV', ['POLKADOT', 'Greetings', 'Greeting from POLKADOT', getCurrentDate()])
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

async function test_scale_codec2() {
  let enc_param1 = u128.enc(-666n);

  console.log('enc_param', enc_param1);
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

async function get_event() {
  console.log("get_event")
  // Subscribe to system events via storage
  api.query.system.events((events) => {
    console.log(`\nReceived ${events.length} events:`);

    // Loop through the Vec<EventRecord>
    events.forEach((record) => {
      console.log('record', record.toHuman());
      // Extract the phase, event and the event types
      const { event, phase } = record;
      const types = event.typeDef;
      // console.log('types', types);

      // Show what we are busy with
      console.log(`\t${event.section}:${event.method}:: (phase=${phase.toString()})`);
      // console.log(`\t\t${event.meta.documentation.toString()}`);

      // Loop through each of the parameters, displaying the type and data
      event.data.forEach((data, index) => {
        console.log('data', data, index);
        console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
      });
    });
  });
}

async function test_decode() {
  let a = '466c69707065723a3a5472616e736665727265640000000000000000000000';
}

function addressTest() {
  // console.log(decodeAddress, encodeAddress);
  let a = [
    164, 193, 205,  72, 150,  30,  15,
    218, 179, 201,  11, 103, 160, 135,
     10, 108,  61, 216, 132,  13, 162,
     26, 108, 188, 242, 121, 211, 133,
    132,  84, 142,  76
  ];
  console.log(toHexString(decodeAddress(("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"))));
  // 0x4a0baeef90dd3c88da26e70f3121b71d037ba4b994cfa9d7d7fd900ca450738b
}

function decodeEvent() {
  // let event = "0x00011cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c011cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c0a000000000000000000000000000000";
  // let t = Struct({
  //   from: 
  // })
}

// 0x010c0100000000000000000000000000000003109a0200000200000000000000000000000000000000201c68746875616e67030000000000000000000000000000000b501867656f72676521000000080c3132330c34353600
// 0x010c0100000000000000000000000000000003109a0200000200000000000000000000000000000000201c68746875616e67030000000000000000000000000000000b501867656f72676521000000080c3132330c34353600
// test_scale_codec()
// test_scale_codec1()
query()
// sendGreeting()
// test_message()

// addressTest()

// let index = 1;
// setInterval(async() => {
//   pushMessage(index);
//   index++;
// }, 5 * 1000);
// pushMessage()
// transferToken()

// get_event()

// test_decode()

// test_scale_codec2()