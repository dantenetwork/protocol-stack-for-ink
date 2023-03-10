import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Abi, ContractPromise } from '@polkadot/api-contract';
import { decodeAddress, encodeAddress } from '@polkadot/keyring';

import fs from 'fs'

// sign and send transaction
export async function sendTransaction(contract, methodName, sender, args) {
    try {
        let value = 0;
        let gasLimit = 10**11;
        const options = { storageDepositLimit: null, gasLimit: -1 }
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
export async function contractCall(contract, method, from, args) {
    let value = 0;
    let gasLimit = 0;
    const { gasConsumed, result, output } = await contract.query[method](from, {value, gasLimit}, ...args);
    return output;
}

export class InkBase {
    constructor(sk, abiPath, c_address) {
        this.sk = sk;
        this.abiPath = abiPath;
        this.contractAddress = c_address;
    }

    async init(api) {
        const keyring = new Keyring({ type: "ecdsa" });
        this.devSender = keyring.addFromSeed(new Uint8Array(Buffer.from(this.sk, 'hex')));

        const contractABIRaw = fs.readFileSync(this.abiPath);
        this.contract = new ContractPromise(api, JSON.parse(contractABIRaw), this.contractAddress);
    }

    // sign and send transaction
    async sendTransaction(methodName, ...args) {
        return await sendTransaction(this.contract, methodName, this.devSender, args);
    }

    // query info from blockchain node
    async contractCall(method, ...args) {
        return await contractCall(this.contract, method, this.devSender.address, args);
    }
}
