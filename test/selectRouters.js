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

const abiFile = fs.readFileSync('../contracts/Protocol/target/ink/metadata.json');
const contract = new ContractPromise(api, JSON.parse(abiFile), process.env.CONTRACT_ADDRESS);

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
      console.log('result', result.isInBlock, result.isFinalized, result.isError, result.isWarning);
      if (result.status.isInBlock) {
        console.log('in a block');
        // console.log(result);
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

// registerRouters()
selectRouters()
