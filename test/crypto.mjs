import { SHA3 } from 'sha3';
import { sha256 } from 'js-sha256';
import keccak256 from 'keccak256';
import elliptic from 'elliptic';
import {encodeAddress, blake2AsU8a} from '@polkadot/util-crypto';
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';

import * as ib from './ink_base.mjs';

const ec = new elliptic.ec('secp256k1');

function signWithKey(msg, hashfunc) {
    const key = ec.keyFromPrivate(Buffer.from("xxxxx", 'hex'));
    const sig = key.sign(hashfunc(msg));
    const n = 32;
    const r = sig.r.toArrayLike(Buffer, 'be', n);
    const s = sig.s.toArrayLike(Buffer, 'be', n);
    console.log(sig.recoveryParam);
    return Buffer.concat([r, s, Buffer.from([sig.recoveryParam + 27])]);
};


async function decodeData() {
    // const sigData = Buffer.from("3c47d2092eb915828fd78e029c22b6de10d172b24185c9a87258d4b162ab898de6a26fa563b751052b3d0b1c0c04c5170138f386d31836b63106848e65cb0fdc", "hex");

    const msg = "hellonika";

    const msgenc = str.enc(msg);

    const sigData = signWithKey(msg, keccak256);
    console.log(sigData.toString('hex'));

    const sigArray = new Uint8Array(sigData);
    console.log("Signature: \n"+ sigArray);
    console.log(sigArray[31]);

    await getPublicKey();

    const hashData = Buffer.from(sha256(msgenc), 'hex');
    const hashArray = new Uint8Array(hashData);
    console.log("Message Hash: \n"+ hashArray);
    console.log(hashData.toString('hex'));
}

async function getPublicKey() {
    const pubKey = "906520128060e4a2ca4c126bdb059d23857d99fe51614533f13917adcfd8e3a1d3e0ce05b272b13740f337d47a06eed052d0c0f8c4316cd615d8d06e11ff8e06";
    const y = "0x" + pubKey.substring(64);
    console.log(y);

    const _1n = BigInt(1);
    let flag = BigInt(y) & _1n ? '03' : '02';
    console.log(flag);

    const x = Buffer.from(pubKey.substring(0, 64), "hex");
    const finalX = Buffer.concat([Buffer.from([flag]), x]);
    const finalXArray = new Uint8Array(finalX);
    console.log("Public Key: \n"+ finalXArray);
    const addrHash = blake2AsU8a(finalXArray);
    console.log(encodeAddress(addrHash));
}

function checkAddress() {
    const inputAddress = new Uint8Array([199, 39, 91, 240, 134, 139, 115, 144, 240, 36, 6, 6, 150, 133, 118, 164, 217, 132, 170, 162, 62, 213, 169, 184, 8, 62, 152, 49, 81, 67, 250, 29]);
    console.log(encodeAddress(inputAddress));
}

async function testKeyring() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });

    const keyring = new Keyring({ type: "ecdsa" });
    const devtest = keyring.addFromSeed(new Uint8Array(Buffer.from('xxxxx', 'hex')));

    console.log(keyring.getPairs()[0].address);
}

async function test_ink_base() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });

    const ink_base = new ib.InkBase(api, 'ecdsa', new Uint8Array(Buffer.from('xxxxx', 'hex')));
}

decodeData()

// getPublicKey()

// checkAddress();

// await testKeyring();

// await test_ink_base();