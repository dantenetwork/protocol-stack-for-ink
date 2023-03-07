"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.encodePolkadotAddress = exports.hashFuncMap = exports.hashBlake2_256 = exports.hashBlake2_128 = exports.hashBlake2_64 = exports.hashSha2_256 = exports.hashKeccak256 = void 0;
var util_crypto_1 = require("@polkadot/util-crypto");
function hashKeccak256(msg) {
    return (0, util_crypto_1.keccakAsU8a)(msg, 256);
}
exports.hashKeccak256 = hashKeccak256;
function hashSha2_256(msg) {
    return (0, util_crypto_1.shaAsU8a)(msg, 256);
}
exports.hashSha2_256 = hashSha2_256;
function hashBlake2_64(msg) {
    return (0, util_crypto_1.blake2AsU8a)(msg, 64);
}
exports.hashBlake2_64 = hashBlake2_64;
function hashBlake2_128(msg) {
    return (0, util_crypto_1.blake2AsU8a)(msg, 128);
}
exports.hashBlake2_128 = hashBlake2_128;
function hashBlake2_256(msg) {
    return (0, util_crypto_1.blake2AsU8a)(msg, 256);
}
exports.hashBlake2_256 = hashBlake2_256;
exports.hashFuncMap = {
    "Keccak256": hashKeccak256,
    "Sha2_256": hashSha2_256,
    "Blake2_64": hashBlake2_64,
    "Blake2_128": hashBlake2_128,
    "Blake2_256": hashBlake2_256
};
exports.encodePolkadotAddress = util_crypto_1.encodeAddress;
