import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { decodeAddress, encodeAddress } from '@polkadot/keyring';
// import { WeightV2 } from '@polkadot/types/interfaces';

import { BN, BN_ONE } from "@polkadot/util";
import fs from 'fs'

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

// sign and send transaction
export async function sendTransaction(api, contract, methodName, sender, args) {
    try {
        let value = 0;
        const options = { 
            storageDepositLimit: null, 
            gasLimit: api.registry.createType('WeightV2', {
                refTime: MAX_CALL_WEIGHT,
                proofSize: PROOFSIZE,
            }),
        };

        const { gasRequired, storageDeposit, result } = await contract.query[methodName](
            sender.address,
            options,
            ...args
        );
        console.log('gasRequired', gasRequired.toString());
        await contract.tx[methodName]({ value, gasLimit: gasRequired }, ...args).signAndSend(sender, (result) => {
            console.log(result.toHuman());
            
            if (result.status.isInBlock) {
                console.log('in a block');
            } else if (result.status.isFinalized) {
                console.log('finalized');
            }
        });

        return 'Ok';
    } catch (e) {
        console.error(e);
    }
}

// query info from blockchain node
export async function contractCall(api, contract, method, from, args) {
    let value = 0;

    const { gasConsumed, result, output } = await contract.query[method](from, 
        {
            value, 
            gasLimit: api.registry.createType('WeightV2', {
                refTime: MAX_CALL_WEIGHT,
                proofSize: PROOFSIZE,
            }),
            storageDepositLimit: null
        }, 
        ...args);

    return {gasConsumed, result, output};
}

export class InkBase {
    constructor(sk, abiPath, c_address) {
        this.sk = sk;
        this.abiPath = abiPath;
        this.contractAddress = c_address;
    }

    async init(api) {
        this.api = api;

        const keyring = new Keyring({ type: "ecdsa" });
        this.devSender = keyring.addFromSeed(new Uint8Array(Buffer.from(this.sk, 'hex')));

        const contractABIRaw = fs.readFileSync(this.abiPath);
        this.contract = new ContractPromise(api, JSON.parse(contractABIRaw), this.contractAddress);
    }

    // sign and send transaction
    async sendTransaction(methodName, ...args) {
        return await sendTransaction(this.api, this.contract, methodName, this.devSender, args);
    }

    // query info from blockchain node
    async contractCall(method, ...args) {
        return await contractCall(this.api, this.contract, method, this.devSender.address, args);
    }
}
