import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { decodeAddress, encodeAddress } from '@polkadot/keyring';

import BN from 'bn.js'
import fs from 'fs'

// sign and send transaction
export async function sendTransaction(gasLimit, contract, methodName, sender, args) {
    try {
        let value = 0;
        const options = { storageDepositLimit: null, gasLimit }
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
export async function contractCall(gasLimit, contract, method, from, args) {
    let value = 0;

    const { gasConsumed, result, output } = await contract.query[method](from, {value, gasLimit}, ...args);

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
        const gasLimit = this.api.registry.createType("WeightV2", {
            refTime: new BN("10000000000"),
            proofSize: new BN("10000000000"),
        });

        return await sendTransaction(gasLimit, this.contract, methodName, this.devSender, args);
    }

    // query info from blockchain node
    async contractCall(method, ...args) {
        const gasLimit = this.api.registry.createType("WeightV2", {
            refTime: new BN("10000000000"),
            proofSize: new BN("10000000000"),
        });

        return await contractCall(gasLimit, this.contract, method, this.devSender.address, args);
    }
}
