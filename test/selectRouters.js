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

}

async function selectRouters() {
    
}
