import oc from './omnichainCrypto.js';
import * as hashfunc from './hashCrypto.mjs';
import {program} from 'commander';

import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';

import * as ib from './ink_base.mjs'

async function localInit() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });

    const rawSeed = 'd9fb0917e1d83e2d42f14f6ac5588e755901150f0aa0953bbf529752e786f50c';
    const abiPath = '../contracts/vv/target/ink/vv.json';
    const contractAddress = '5EN8D4PEjkJY6wbDfqHCURTZ3vrS5kJiCJjp1gDv9GW9gnD3';

    const ibinst = new ib.InkBase(rawSeed, abiPath, contractAddress);
    await ibinst.init(api);

    return ibinst;
}

async function sign(msg, sk, ec_name, hash_name) {
    // const ec = new elliptic.ec('secp256k1');
    // const key = ec.keyFromPrivate(Buffer.from("d9fb0917e1d83e2d42f14f6ac5588e755901150f0aa0953bbf529752e786f50c", 'hex'));
    // console.log(Buffer.from(key.getPublic().encode()).toString('hex'));

    const signService = new oc.OmnichainCrypto(hashfunc.hashFuncMap[hash_name], ec_name, sk);
    const signature_content = signService.sign2hexstringrecovery(msg);
    console.log(signature_content);

    console.log("Public Key:\n"+signService.getPublic());

    var compressed = signService.getPublicCompressed();
    console.log("Compressed Public Key:\n"+compressed);
    const pkArray = new Uint8Array(Buffer.from(compressed, 'hex'));
    console.log("Polkadot Address: ");
    console.log(hashfunc.encodePolkadotAddress(hashfunc.hashFuncMap['Blake2_256'](new Uint8Array(Buffer.from(compressed, 'hex')))));
    
    console.log(signService.verify(msg, signature_content));
}

async function activeCall() {
    const ibinst = await localInit();

    await ibinst.sendTransaction('flip');

    console.log("*********************Everything for a transaction call is active.**********************");
}

async function activeQuery() {
    const ibinst = await localInit();

    console.log(await ibinst.contractCall('getRand', 73));

    console.log("*********************Everything for a transaction query is active.**********************");
}

function list(val) {
    if (val == undefined) {
        return [];
    } else {
        return val.split(',');
    }
}

function list_line(val) {
    return val.split('|');
}

async function commanders() {
    program
        .version('Test Tools for VV Stage. v0.0.1')
        .option('--sign <message>,<private key>,<elliptic name>,<hash func name>', 'sign a message with designated elliptic name and hash function name.', list)
        .option('--active-call', 'Check if everything for a transaction call is OK.', list)
        .option('--active-query', 'Check if everything for a query is OK.', list)
        .parse(process.argv);
        
    if (program.opts().sign) {
        if (program.opts().sign.length != 4) {
            console.log('4 arguments are needed, but ' + program.opts().sign.length + ' provided');
            return;
        }

        console.log('sign a message: '+program.opts().sign[0]);

        await sign(program.opts().sign[0], program.opts().sign[1], program.opts().sign[2], program.opts().sign[3]);
    } else if (program.opts().activeCall) {
        if (program.opts().activeCall.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().sign.length + ' provided');
            return;
        }

        await activeCall();

    } else if (program.opts().activeQuery) {
        if (program.opts().activeQuery.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().sign.length + ' provided');
            return;
        }

        await activeQuery();
    }
}

await commanders();
