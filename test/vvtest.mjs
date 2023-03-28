import oc from './omnichainCrypto.js';
import * as hashfunc from './hashCrypto.mjs';
import {program} from 'commander';
import secp266k1 from 'secp256k1';

import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';

import * as ib from './ink_base.mjs';
import * as td from './typedefines.mjs';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';

const rawSeed = 'd9fb0917e1d83e2d42f14f6ac5588e755901150f0aa0953bbf529752e786f50c';

async function localInit() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });

    const abiPath = '../contracts/vv/target/ink/vv.json';
    const contractAddress = '5HkobVJEbBzSeRnY5XTL3vq3wmDSTNCS4DB9EXTXTkG7BT3f';

    const ibinst = new ib.InkBase(rawSeed, abiPath, contractAddress);
    await ibinst.init(api);

    return ibinst;
}

async function testSignAndAddress() {
    // const ec = new elliptic.ec('secp256k1');
    // const key = ec.keyFromPrivate(Buffer.from("d9fb0917e1d83e2d42f14f6ac5588e755901150f0aa0953bbf529752e786f50c", 'hex'));
    // console.log(Buffer.from(key.getPublic().encode()).toString('hex'));
    const msg = 'hello';

    const signService = new oc.OmnichainCrypto(hashfunc.hashFuncMap['Keccak256'], 'secp256k1', rawSeed);
    const signature_content = signService.sign2hexstringrecovery(msg);
    console.log(signature_content);

    console.log("Public Key:\n"+signService.getPublic());

    // var compressed = signService.getPublicCompressed();
    // console.log("Compressed Public Key:\n"+compressed);
    // const pkArray = new Uint8Array(Buffer.from(compressed, 'hex'));
    // console.log("Polkadot Address: ");
    // console.log(hashfunc.encodePolkadotAddress(hashfunc.hashFuncMap['Blake2_256'](new Uint8Array(Buffer.from(compressed, 'hex')))));
    
    // console.log(signService.verify(msg, signature_content));

    let sign_data = new Uint8Array(Buffer.from(signature_content, 'hex'));
    console.log(sign_data);

    let recover_pk = secp266k1.ecdsaRecover(sign_data.subarray(0, 64), sign_data[64] - 27, hashfunc.hashFuncMap['Keccak256'](msg),false);
    console.log(Buffer.from(recover_pk).toString('hex'));
}

async function sign(msg, sk, ec_name, hash_name) {
    const signService = new oc.OmnichainCrypto(hashfunc.hashFuncMap[hash_name], ec_name, sk);
    const signature_content = signService.sign2bufferrecovery(msg);

    // // console.log("Public Key:\n"+signService.getPublic());
    // var compressed = signService.getPublicCompressed();
    // // console.log("Compressed Public Key:\n"+compressed);
    // compressed = hashfunc.hashFuncMap['Blake2_256'](new Uint8Array(Buffer.from(compressed, 'hex')));
    // console.log(compressed);    // This is just the on-chain account  

    return signature_content;
}

async function activeCall() {
    const ibinst = await localInit();

    await ibinst.sendTransaction('flip');

    console.log("*********************Everything for a transaction call is active.**********************");
}

async function activeQuery() {
    const ibinst = await localInit();

    const {gasConsumed, result, output} = await ibinst.contractCall('get');

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

    console.log("*********************Everything for a transaction query is active.**********************");
}

async function testStructure() {
    const ibinst = await localInit();

    const {gasConsumed, result, output} = await ibinst.contractCall('testStructure');

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

    console.log("*********************Everything for a transaction query is active.**********************");
}

async function testVVSignature() {
    let payload = new td.MsgPayload();
    await payload.addItem(new td.MsgItem("hello", td.MsgType.InkString, "nika"));
    await payload.addItem(new td.MsgItem("Alice", td.MsgType.InkU128Array, [BigInt('123456789'), BigInt('987654321')]));

    let sqos = [];
    sqos.push(new td.SQoSItem(td.SQoSType.Challenge, [1,2,3,4,5,6,7,8]).item);
    sqos.push(new td.SQoSItem(td.SQoSType.Isolation, [9,8,7,6]).item);

    let sess = td.createSession(1, 2, Buffer.from("hello"), Buffer.from("nika"), []);

    let recved_message = {
        id: BigInt(1),
        from_chain: "Polkadot",
        to_chain: "Ethereum",
        sender: (Buffer.alloc(32, 'a')).toJSON().data,
        signer: Array.from(Buffer.alloc(32, 'a')),
        sqos: sqos,
        contract: (Buffer.alloc(32, 'a')).toJSON().data,
        action: [1,2,3,4],
        data: Array.from(await payload.encode()),
        session: sess,
    };

    // console.log(td.transferRecvedMsg2RawData(recved_message));
    // console.log((Buffer.from(hashfunc.hashFuncMap['Keccak256'](td.transferRecvedMsg2RawData(recved_message))).toString('hex')));

    const ibinst = await localInit();

    const {gasConsumed, result, output} = await ibinst.contractCall('getRecvHash', recved_message);

    // The actual result from RPC as `ContractExecResult`
    // console.log(result.toHuman());
    // gas consumed
    // console.log(gasConsumed.toHuman());
    // check if the call was successful
    if (result.isOk) {
        // should output 123 as per our initial set (output here is an i32)
        console.log('Success', output.toHuman());
        
        let signature = await sign(output.toHuman().Ok[0], rawSeed, 'secp256k1', 'Keccak256');

        let vv_message = {
            recved_msg: recved_message,
            signature: Array.from(signature)
        };

        // console.log(td.IVVMessageRecved.enc(vv_message));
        const second_result = await ibinst.contractCall('submitVvMessage', vv_message);
        console.log(second_result.result.toHuman());

        if (second_result.result.isOk) {
            console.log('Success', second_result.output.toHuman());
        }

    } else {
        console.error('Error', result.asErr.toHuman());
    }
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
        .option('--test-structure', 'check the data structure in `js`', list)
        .option('--test-vv-signature', 'check the data structure in `js`', list)
        .option('--test-sign-aa', 'check the data structure in `js`', list)
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
    } else if (program.opts().testStructure) {
        if (program.opts().testStructure.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().sign.length + ' provided');
            return;
        }

        await testStructure();
    } else if (program.opts().testVvSignature) {
        if (program.opts().testVvSignature.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().sign.length + ' provided');
            return;
        }

        await testVVSignature();
    } else if (program.opts().testSignAa) {
        if (program.opts().testSignAa.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().sign.length + ' provided');
            return;
        }

        await testSignAndAddress();
    }
}

await commanders();
