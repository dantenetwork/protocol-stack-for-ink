import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';
import fs from 'fs';
import 'dotenv/config'


const provider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({provider});

const MessageDetail = Struct({
  name: str,
  age: u32,
  phones: Vector(str)
});

const InfoEvent = Struct({
  topic_name : str,
  instance : Option(MessageDetail)
});

const keyring = new Keyring({ type: 'sr25519' });
let data = fs.readFileSync('./.secret/keyPair.json');
const sender = keyring.addFromJson(JSON.parse(data.toString()));
sender.decodePkcs8(process.env.PASSWORD);

const abiFile = fs.readFileSync('../contracts/Protocol/target/ink/metadata.json');
const contract = new ContractPromise(api, JSON.parse(abiFile), process.env.CONTRACT_ADDRESS);
const abi = new Abi(JSON.parse(abiFile));

// console.log(InfoEvent.dec('0x01285375706572204e696b61014c726f7574657273207265676973746572656421'));

async function registerRouters() {
    const value = 0; // only useful on isPayable messages
    // NOTE the apps UI specified these in mega units
    const gasLimit = -1;

    // cres = {"routers": cres};
    // console.log(cres);

    let cres = Array.from({length: 10}, v=> Math.floor(Math.random() * 100 + 1));

    await contract.tx
    .randomRegisterRouters({ value, gasLimit }, cres)
    .signAndSend(sender, (result) => {
      // console.log('result', result.isInBlock, result.isFinalized, result.isError, result.isWarning);
      // console.log(result.events);
      if (result.status.isInBlock) {
        result.events.forEach(({ event, topics }) => {
          if (api.events.contracts.ContractEmitted.is(event)) {
              // console.log(topics.toHuman());
              const [account_id, contract_evt] = event.data;
              // console.log(event.index.toHuman());
              if (account_id.toString() == process.env.CONTRACT_ADDRESS) {
                const decoded = abi.decodeEvent(contract_evt);
                if (decoded.event.identifier == 'InfoEvent') {
                    console.log(InfoEvent.dec(contract_evt.slice(1, contract_evt.length)));
                }
              }
          }
        });
      } else if (result.status.isFinalized) {
        console.log('finalized');
      }
    });
}

async function selectRouters() {
    const value = 0; // only useful on isPayable messages
    // NOTE the apps UI specified these in mega units
    const gasLimit = -1;

    const epoch = 10000;

    const { gasConsumed, result, output } = await contract.query['selectionStatistic'](sender.address, {value, gasLimit }, epoch);

    // The actual result from RPC as `ContractExecResult`
    console.log(result.toHuman());
    
    // gas consumed
    console.log(gasConsumed.toHuman());

    // check if the call was successful
    if (result.isOk) {
        // should output 123 as per our initial set (output here is an i32)
        // console.log('Success', output.toHuman());
        let res = output.toHuman();
        // console.log(res[res.length - 1].high);
        let totalCres = parseInt(res[res.length - 1].high.replace(/,/g,''));
        // console.log(totalCres);
        res.forEach(element => {
          // console.log(element.selected);
          console.log(`probability: ${parseInt(element.cre.replace(/,/g,'')) * 100 / totalCres}%`, `frequency: ${parseInt(element.selected.replace(/,/g,'')) * 100 / epoch}%`);
        });
    } else {
        console.error('Error', result.asErr);
    }
}

registerRouters()
// selectRouters()
