import oc from './omnichainCrypto.js';
import * as hashfunc from './hashCrypto.mjs';
import * as addrAd from './addressAdapter.mjs';
import {program} from 'commander';
import secp266k1 from 'secp256k1';

import Web3 from 'web3'

import {ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { extractPublicKey, personalSign, } from '@metamask/eth-sig-util';
import {cryptoWaitReady, decodeAddress, signatureVerify} from '@polkadot/util-crypto';
import { stringToHex, u8aToHex } from "@polkadot/util";
// import { cryptoWaitReady } from '@polkadot/wasm-crypto';
// import { cryptoWaitReady } from '@polkadot/util-crypto';
// import { Signer } from '@polkadot/api/types';

import * as ib from './ink_base.mjs';
import * as td from './typedefines.mjs';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';

const rawSeed = '8ebe5d1ff8f4880b5cff6be072441bad28a981c80ffeb755bba028fac13876f5';

async function localInit(signAl = 'ethereum') {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });

    const abiPath = '../contracts/vv/target/ink/vv.json';
    const contractAddress = '5Dy6WZ3r8ddtm7Cky4JDqvoP13GnSdqeMCpn1NzH62pvVRb9';

    const ibinst = new ib.InkBase(rawSeed, abiPath, contractAddress, signAl);
    await ibinst.init(api);

    return ibinst;
}

async function testSignAndAddress() {
    const msg = 'hello';

    // console.log(Buffer.from(msg, 'utf8').toString('hex'));

    const signService = new oc.OmnichainCrypto(hashfunc.hashFuncMap['Keccak256'], 'secp256k1', rawSeed);
    const signature_content = signService.sign2hexstringrecovery(msg);
    console.log(signature_content);

    console.log("Public Key:\n"+signService.getPublic());

    let pubkey = '0x'+signService.getPublic();

    var compressed = signService.getPublicCompressed();
    // var compressed = oc.publicKeyCompress('c8e7118c4c65ba2b832dd77be18a65b3f019d1bff34dcfa3b1879a6e651b32fc047381c98bf48575a181fdb584a80dd872c1e4208736233af12b923b19c1c19a');
    console.log("Compressed Public Key:\n"+compressed);
    const pkArray = new Uint8Array(Buffer.from(compressed, 'hex'));
    console.log("Polkadot Address: ");
    console.log(hashfunc.encodePolkadotAddress(hashfunc.hashFuncMap['Blake2_256'](new Uint8Array(Buffer.from(compressed, 'hex')))));
    // console.log(hashfunc.encodePolkadotAddress(`0x${Buffer.from(compressed).toString('hex')}`, 42, 'blake2'));
    console.log(await addrAd.polkadotAddressFromPubKey(pubkey));

    console.log("EVM Address: ");
    const web3 = new Web3();
    let emv_address = web3.eth.accounts.privateKeyToAccount(rawSeed).address;
    console.log(emv_address);
    console.log(hashfunc.evmAddressToAddress(emv_address, 42, 'blake2'));

    
    console.log(signService.verify(msg, signature_content));

    let sign_data = new Uint8Array(Buffer.from(signature_content, 'hex'));
    // let sign_data = new Uint8Array(Buffer.from('6926054a3cc91712de704af9a588bcf5bd2e74d02a2dc0dcfdda1a929bcb7f1951d12c9cd620a10532479cc763983353b893706589e2ae4f5304bf1149a2226301', 'hex'));
    console.log(sign_data);

    let recover_pk = secp266k1.ecdsaRecover(sign_data.subarray(0, 64), sign_data[64] - 27, hashfunc.hashFuncMap['Keccak256'](msg),false);
    // let recover_pk = secp266k1.ecdsaRecover(sign_data.subarray(0, 64), sign_data[64], hashfunc.hashFuncMap['Blake2_256'](msg),true);
    console.log(Buffer.from(recover_pk).toString('hex'));
}

async function testEthSign() {
    let helloWorldMessage = `0x${Buffer.from('hello', 'utf-8').toString('hex')}`;

    let signature = personalSign({ privateKey: rawSeed, data: helloWorldMessage });

    let pk = extractPublicKey({
        data: helloWorldMessage,
        signature: signature,
    });

    console.log(pk);
}

async function testPolkadotSign(signAl, wrapType) {

    let inbs = await localInit(signAl);

    let message = 'hello';

    if (wrapType === 'ethereum') {
        message = `\x19Ethereum Signed Message:\n${message.length}hello`;
    } else if (wrapType === 'polkadot') {
        message = '<Bytes>hello</Bytes>';
    }

    // Create a message to sign
    // const message = '<Bytes>hello</Bytes>';
    // const message = '\x19Ethereum Signed Message:\nhello';

    const signature = inbs.devSender.sign(message);

    console.log(Buffer.from(signature).toString('hex'));

    const s2 = inbs.devSender.sign(message);
    console.log(Buffer.from(s2).toString('hex'));

    const isValidSignature = (signedMessage, signature, address) => {
        const publicKey = decodeAddress(address);
        const hexPublicKey = u8aToHex(publicKey);

        console.log(address);
        console.log(hexPublicKey);
      
        return signatureVerify(signedMessage, signature, hexPublicKey).isValid;
    };

    await cryptoWaitReady();
    const isValid = isValidSignature(
        message,
        signature,
        inbs.devSender.address
    );
    console.log(isValid)
}

async function signLocal(msg, sk, ec_name, hash_name) {
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
        
        let signature = await signLocal(output.toHuman().Ok[0], rawSeed, 'secp256k1', 'Keccak256');

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
        .option('--sign-local <message>,<private key>,<elliptic name>,<hash func name>', 'sign a message with designated elliptic name and hash function name.', list)
        .option('--active-call', 'Check if everything for a transaction call is OK.', list)
        .option('--active-query', 'Check if everything for a query is OK.', list)
        .option('--test-structure', 'check the data structure in `js`', list)
        .option('--test-vv-signature', 'check the data structure in `js`', list)
        .option('--test-sign-aa', 'check the signature and recover', list)
        .option('--test-eth-sign', 'check the eth-sign-util', list)
        .option('--test-polkadot-sign <sign algorithem>,<wrap type>', 'check the polkadot sign with wallet. <sign algorithm>: `ethereum`, `ecdsa`, `sr25519`, `ed25519`. <wrap type>: `polkadot`, `ethereum`.', list)
        .parse(process.argv);
        
    if (program.opts().signLocal) {
        if (program.opts().signLocal.length != 4) {
            console.log('4 arguments are needed, but ' + program.opts().signLocal.length + ' provided');
            return;
        }

        console.log('sign a message: '+program.opts().sign[0]);

        await signLocal(program.opts().sign[0], program.opts().sign[1], program.opts().sign[2], program.opts().sign[3]);
    } else if (program.opts().activeCall) {
        if (program.opts().activeCall.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().activeCall.length + ' provided');
            return;
        }

        await activeCall();

    } else if (program.opts().activeQuery) {
        if (program.opts().activeQuery.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().activeQuery.length + ' provided');
            return;
        }

        await activeQuery();
    } else if (program.opts().testStructure) {
        if (program.opts().testStructure.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().testStructure.length + ' provided');
            return;
        }

        await testStructure();
    } else if (program.opts().testVvSignature) {
        if (program.opts().testVvSignature.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().testVvSignature.length + ' provided');
            return;
        }

        await testVVSignature();
    } else if (program.opts().testSignAa) {
        if (program.opts().testSignAa.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().testSignAa.length + ' provided');
            return;
        }

        await testSignAndAddress();
    } else if (program.opts().testEthSign) {
        if (program.opts().testEthSign.length != 0) {
            console.log('0 arguments are needed, but ' + program.opts().testEthSign.length + ' provided');
            return;
        }

        await testEthSign();
    } else if (program.opts().testPolkadotSign) {
        if (program.opts().testPolkadotSign.length != 2) {
            console.log('2 arguments are needed, but ' + program.opts().testPolkadotSign.length + ' provided');
            return;
        }

        await testPolkadotSign(program.opts().testPolkadotSign[0], program.opts().testPolkadotSign[1]);
    }
}

await commanders();
