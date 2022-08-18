import {sha256} from 'js-sha256';
import elliptic from 'elliptic';
import {encodeAddress, blake2AsU8a} from '@polkadot/util-crypto';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';

const ec = new elliptic.ec('secp256k1');

function signWithKey(msg) {
    const key = ec.keyFromPrivate(Buffer.from("d9fb0917e1d83e2d42f14f6ac5588e755901150f0aa0953bbf529752e786f50c", 'hex'));
    const sig = key.sign(hashMsg(msg));
    const n = 32;
    const r = sig.r.toArrayLike(Buffer, 'be', n);
    const s = sig.s.toArrayLike(Buffer, 'be', n);
    console.log(sig.recoveryParam);
    return Buffer.concat([r, s, Buffer.from([sig.recoveryParam + 27])]);
  };

function hashMsg(msg) {
    return sha256(msg);
  };

async function decodeData() {
    // const sigData = Buffer.from("3c47d2092eb915828fd78e029c22b6de10d172b24185c9a87258d4b162ab898de6a26fa563b751052b3d0b1c0c04c5170138f386d31836b63106848e65cb0fdc", "hex");

    const msg = "hello nika";

    const msgenc = str.enc(msg);

    const sigData = signWithKey(msg);
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
    const pubKey = "4659091d1840f761ace31f9db3e2e34efc640e109c3c2f6e573a104ee69614720e489d544950f3eaa7bb0174077aaf521da91ad8c3d8a2527675cb72927fd4bc";
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

decodeData()

// getPublicKey()

// checkAddress();