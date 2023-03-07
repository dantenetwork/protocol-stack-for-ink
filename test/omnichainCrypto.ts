import * as elliptic from 'elliptic';
import KeyPair from 'elliptic/lib/elliptic/ec/key';

export class OmnichainCrypto {

    keyPair: KeyPair;
    hashFun: (msg: string| Uint8Array) => Uint8Array;
    ec: any;

    constructor(hashFun: (msg: string| Uint8Array) => Uint8Array, curveName: string, sk?: string) {
        this.hashFun = hashFun;
        this.ec = new elliptic.ec(curveName);

        if (typeof sk! == 'undefined') {
            this.keyPair = this.ec.genKeyPair();
        } else {
            this.keyPair = this.ec.keyFromPrivate(sk!);
        }

        // if (this.pubKey.length === 64) {
        //     this.pubKey = Buffer.concat([Buffer.from([4]), this.pubKey]);
        // } else if (this.pubKey.length === 33 || this.pubKey.length === 0) {
        //     // do nothing
        // } else {
        //     throw("Invalid public key!");
        // }

        // if ((this.priKey.length != 0) && (this.priKey.length != 32)) {
        //     throw("Invalid private key!");
        // }
    }

    getPrivate = () => {
        return this.keyPair.getPrivate();
    }

    getPublic = () => {
        // The length of the uncompressed public key is 65, and the first `04` indicates uncompressed
        return this.keyPair.getPublic('hex');
    }

    getPublicCompressed = () => {
        const pubKey = this.keyPair.getPublic('hex').toString();
        return Buffer.from(publicKeyCompress(pubKey.substring(2))).toString('hex');
    }

    sign2buffer= (msg: string | Uint8Array): Buffer => {
        const sig = this.keyPair.sign(this.hashFun(msg));
        const n = 32;
        const r = sig.r.toArrayLike(Buffer, 'be', n);
        const s = sig.s.toArrayLike(Buffer, 'be', n);
        return Buffer.concat([r, s]);
    };

    sign2hexstring = (msg: string | Uint8Array): string => {
        return this.sign2buffer(msg).toString('hex');
    };

    sign2bufferrecovery = (msg: string | Uint8Array): Buffer => {
        const sig = this.keyPair.sign(this.hashFun(msg));
        const n = 32;
        const r = sig.r.toArrayLike(Buffer, 'be', n);
        const s = sig.s.toArrayLike(Buffer, 'be', n);
        return Buffer.concat([r, s, Buffer.from([sig.recoveryParam + 27])]);
    };

    sign2hexstringrecovery = (msg: string | Uint8Array): string => {
        return this.sign2bufferrecovery(msg).toString('hex');
    }
 
    sign = (msg: string | Uint8Array): elliptic.ec.Signature => {
        const sig = this.keyPair.sign(this.hashFun(msg));
        return sig;
    }

    verify = (msg: string | Uint8Array,
            signature: string | elliptic.ec.Signature) => {
    
        const msgHash = this.hashFun(msg);
        return this.keyPair.verify(msgHash, signature);
    }
}

export function publicKeyCompress(pubKey: string) {
    if (pubKey.length == 128) {
        const y = "0x" + pubKey.substring(64);
        // console.log(y);

        const _1n = BigInt(1);
        let flag = BigInt(y) & _1n ? '03' : '02';
        // console.log(flag);

        const x = Buffer.from(pubKey.substring(0, 64), "hex");
        const finalX = Buffer.concat([Buffer.from(flag, 'hex'), x]);
        const finalXArray = new Uint8Array(finalX);
        // console.log("Public Key: \n"+ finalXArray);

        return finalXArray;
    } else {
        throw("Invalid public key length!" + pubKey.length);
    }
}

