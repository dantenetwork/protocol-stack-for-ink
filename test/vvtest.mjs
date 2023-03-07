import oc from './omnichainCrypto.js';
import hashfunc from './hashCrypto.js';
import {program} from 'commander';

// import { SHA3 } from 'sha3';
// import { sha256 } from 'js-sha256';
// import keccak256 from 'keccak256';

// async function testHashFunc() {
//     console.log(hashfunc.hashFuncMap["Keccak256"]("hello nika"));
// }

// await testHashFunc();

async function sign(msg, sk, ec_name, hash_name) {
    // const ec = new elliptic.ec('secp256k1');
    // const key = ec.keyFromPrivate(Buffer.from("d9fb0917e1d83e2d42f14f6ac5588e755901150f0aa0953bbf529752e786f50c", 'hex'));
    // console.log(Buffer.from(key.getPublic().encode()).toString('hex'));

    const signService = new oc.OmnichainCrypto(hashfunc.hashFuncMap[hash_name], ec_name, sk);
    console.log(signService.sign2hexstringrecovery(msg));

    console.log("Public Key:\n"+signService.getPublic());

    var compressed = signService.getPublicCompressed();
    console.log("Compressed Public Key:\n"+compressed);
    const pkArray = new Uint8Array(Buffer.from(compressed, 'hex'));
    console.log("Polkadot Address: ");
    console.log(hashfunc.encodePolkadotAddress(hashfunc.hashFuncMap['Blake2_256'](new Uint8Array(Buffer.from(compressed, 'hex')))));
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
        .parse(process.argv);
        
    if (program.opts().sign) {
        // if (program.opts().regrouter.length != 0) {
        //     console.log('0 arguments are needed, but ' + program.opts().regrouter.length + ' provided');
        //     return;
        // }

        console.log('sign a message: '+program.opts().sign[0]);

        await sign(program.opts().sign[0], program.opts().sign[1], program.opts().sign[2], program.opts().sign[3]);
    }
}

await commanders();
