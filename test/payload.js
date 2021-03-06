import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';
import fs from 'fs';
import 'dotenv/config'

const provider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({provider});

const InkAddressData = Struct({
  ink_address: Option(Vector(u8)),
  general_address: Option(str),
});

const keyring = new Keyring({ type: 'sr25519' });
let data = fs.readFileSync('./.secret/keyPair.json');
const sender = keyring.addFromJson(JSON.parse(data.toString()));
sender.decodePkcs8(process.env.PASSWORD);

const abiFile = fs.readFileSync('../contracts/callee/target/ink/metadata.json');
const contract = new ContractPromise(api, JSON.parse(abiFile), process.env.CALLEE_CONTRACT);

// test encoder
const payloadABI = new Abi(JSON.parse(abiFile));
// const payed = payloadABI.findMessage('getMessage').toU8a([{'items': null, 'vecs': [{'n': 'Nika', 't': 'InkU16', 'v': '0x99887766'}]}]);
// const payedDecode = payloadABI.findMessage('getMessage').fromU8a(payed.subarray(5));
// console.log(payedDecode.args[0].toHuman());
// console.log(payedDecode.args[0].toJSON().vecs[0]);

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
    const { gasConsumed, result, output } = await contract.query['getPayload'](sender.address, {value, gasLimit }, 
                                                                                { items: [ { n: 'nika', tv: { InkAddress: { ink_address: [1,2,3,4], general_address: 'Hello Nika' } } } ] });
    
    // The actual result from RPC as `ContractExecResult`
    console.log(result.toHuman());
    
    // gas consumed
    console.log(gasConsumed.toHuman());
  
    // check if the call was successful
    if (result.isOk) {
      // should output 123 as per our initial set (output here is an i32)
      console.log('Success', output);
      if (output.items){
        console.log(output.items.toHuman());
      }

      if (output.vecs){
        console.log(output.vecs.toHuman());
        console.log(parseInt(output.vecs.toJSON()[0].v[0], 16) + 1);
      }

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

async function queryNewDetail() {
    const value = 0; // only useful on isPayable messages
    // NOTE the apps UI specified these in mega units
    const gasLimit = -1;
    
    // const storage_deposit_limit = 3n * 1000000n;
    
    // Perform the actual read (with one param, which is an user defined struct)
    // (We perform the send from an account, here using address created from a Json)
    // const { gasConsumed, result, output } = await contract.query['submitMessage'](sender.address, { value, gasLimit }, 
    //                                         {"name": "Nika", "age": 18, "phones": ["123", "456"]});

    // const calleeEncode = calleeABI.findMessage('encode_user_defined_struct').toU8a([{"name": "Nika", "age": 18, "phones": ["123", "456"]}]);
    const { gasConsumed, result, output } = await contract.query['detailItemSR'](sender.address, {value, gasLimit }, 
                                                                              {n: 'Hello Nika', tv: {IOU64Array: [77, 88]}});
    
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

  query();
  // queryNewDetail();