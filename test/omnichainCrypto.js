"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.publicKeyCompress = exports.OmnichainCrypto = void 0;
var elliptic = require("elliptic");
var OmnichainCrypto = /** @class */ (function () {
    function OmnichainCrypto(hashFun, curveName, sk) {
        var _this = this;
        this.getPrivate = function () {
            return _this.keyPair.getPrivate();
        };
        this.getPublic = function () {
            // The length of the uncompressed public key is 65, and the first `04` indicates uncompressed
            return _this.keyPair.getPublic('hex');
        };
        this.getPublicCompressed = function () {
            var pubKey = _this.keyPair.getPublic('hex').toString();
            return Buffer.from(publicKeyCompress(pubKey.substring(2))).toString('hex');
        };
        this.sign2buffer = function (msg) {
            var sig = _this.keyPair.sign(_this.hashFun(msg));
            var n = 32;
            var r = sig.r.toArrayLike(Buffer, 'be', n);
            var s = sig.s.toArrayLike(Buffer, 'be', n);
            return Buffer.concat([r, s]);
        };
        this.sign2hexstring = function (msg) {
            return _this.sign2buffer(msg).toString('hex');
        };
        this.sign2bufferrecovery = function (msg) {
            var sig = _this.keyPair.sign(_this.hashFun(msg));
            var n = 32;
            var r = sig.r.toArrayLike(Buffer, 'be', n);
            var s = sig.s.toArrayLike(Buffer, 'be', n);
            return Buffer.concat([r, s, Buffer.from([sig.recoveryParam + 27])]);
        };
        this.sign2hexstringrecovery = function (msg) {
            return _this.sign2bufferrecovery(msg).toString('hex');
        };
        this.sign = function (msg) {
            var sig = _this.keyPair.sign(_this.hashFun(msg));
            return sig;
        };
        this.verify = function (msg, signature) {
            var sig;
            if (typeof signature == 'string') {
                if (signature.length == 130) {
                    sig = {
                        r: signature.substring(0, 64),
                        s: signature.substring(64, 128),
                        v: signature.substring(128, 130)
                    };
                }
                else if (signature.length == 128) {
                    sig = {
                        r: signature.substring(0, 64),
                        s: signature.substring(64, 128)
                    };
                }
                else {
                    throw ("Invalid signature length: " + signature.length);
                }
            }
            else {
                sig = signature;
            }
            var msgHash = _this.hashFun(msg);
            return _this.keyPair.verify(msgHash, sig);
        };
        this.hashFun = hashFun;
        this.ec = new elliptic.ec(curveName);
        if (typeof sk == 'undefined') {
            this.keyPair = this.ec.genKeyPair();
        }
        else {
            this.keyPair = this.ec.keyFromPrivate(sk);
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
    return OmnichainCrypto;
}());
exports.OmnichainCrypto = OmnichainCrypto;
function publicKeyCompress(pubKey) {
    if (pubKey.length == 128) {
        var y = "0x" + pubKey.substring(64);
        // console.log(y);
        var _1n = BigInt(1);
        var flag = BigInt(y) & _1n ? '03' : '02';
        // console.log(flag);
        var x = Buffer.from(pubKey.substring(0, 64), "hex");
        var finalX = Buffer.concat([Buffer.from(flag, 'hex'), x]);
        var finalXArray = new Uint8Array(finalX);
        // console.log("Public Key: \n"+ finalXArray);
        return finalXArray;
    }
    else {
        throw ("Invalid public key length!" + pubKey.length);
    }
}
exports.publicKeyCompress = publicKeyCompress;
