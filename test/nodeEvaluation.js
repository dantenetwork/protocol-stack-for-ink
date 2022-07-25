import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';
import fs from 'fs';
import 'dotenv/config'

const EvaluateResult = Struct({
    behavior_type: str,
    results: Vector(u32),
});

const provider = new WsProvider("ws://127.0.0.1:9944");
const api = await ApiPromise.create({provider});

const keyring = new Keyring({ type: 'sr25519' });
let data = fs.readFileSync('./.secret/keyPair.json');
const sender = keyring.addFromJson(JSON.parse(data.toString()));
sender.decodePkcs8(process.env.PASSWORD);

const abiFile = fs.readFileSync('../contracts/algorithm/target/ink/metadata.json');
const contract = new ContractPromise(api, JSON.parse(abiFile), process.env.ALG_CONTRACT);
const abi = new Abi(JSON.parse(abiFile));

async function do_honest() {
    const value = 0; // only useful on isPayable messages
    // NOTE the apps UI specified these in mega units
    const gasLimit = -1;

    await contract.tx.doHonest({ value, gasLimit }, 0, 20)
                    .signAndSend(sender, (result) => {
                        console.log('result', result.isInBlock, result.isFinalized, result.isError, result.isWarning);
                        // console.log(result.events);
                        if (result.status.isInBlock) {
                            console.log("do honest successed!");
                            result.events.forEach(({ event, topics }) => {
                                if (api.events.contracts.ContractEmitted.is(event)) {
                                    // console.log(topics.toHuman());
                                    const [account_id, contract_evt] = event.data;
                                    // console.log(event.index.toHuman());
                                    if (account_id.toString() == process.env.ALG_CONTRACT) {
                                    const decoded = abi.decodeEvent(contract_evt);
                                    if (decoded.event.identifier == 'EvaluateResult') {
                                        console.log(EvaluateResult.dec(contract_evt.slice(1, contract_evt.length)));
                                    }
                                    }
                                }
                            });
                        } else if (result.status.isFinalized) {
                          console.log('finalized');
                        }
                      });
}

async function do_evil() {
    const value = 0; // only useful on isPayable messages
    // NOTE the apps UI specified these in mega units
    const gasLimit = -1;

    await contract.tx.doEvil({ value, gasLimit }, 0, 20)
                    .signAndSend(sender, (result) => {
                        console.log('result', result.isInBlock, result.isFinalized, result.isError, result.isWarning);
                        // console.log(result.events);
                        if (result.status.isInBlock) {
                            console.log("do honest successed!");
                            result.events.forEach(({ event, topics }) => {
                                if (api.events.contracts.ContractEmitted.is(event)) {
                                    // console.log(topics.toHuman());
                                    const [account_id, contract_evt] = event.data;
                                    // console.log(event.index.toHuman());
                                    if (account_id.toString() == process.env.ALG_CONTRACT) {
                                    const decoded = abi.decodeEvent(contract_evt);
                                    if (decoded.event.identifier == 'EvaluateResult') {
                                        console.log(EvaluateResult.dec(contract_evt.slice(1, contract_evt.length)));
                                    }
                                    }
                                }
                            });
                        } else if (result.status.isFinalized) {
                          console.log('finalized');
                        }
                      });
}

// do_honest()
do_evil();
