import {encodeAddress, blake2AsU8a, keccakAsU8a, shaAsU8a} from '@polkadot/util-crypto';

export function hashKeccak256(msg: string | Uint8Array) {
    return keccakAsU8a(msg, 256);
}

export function hashSha2_256(msg: string | Uint8Array) {
    return shaAsU8a(msg, 256);
}

export function hashBlake2_64(msg: string | Uint8Array) {
    return blake2AsU8a(msg, 64);
}

export function hashBlake2_128(msg: string | Uint8Array) {
    return blake2AsU8a(msg, 128);
}

export function hashBlake2_256(msg: string | Uint8Array) {
    return blake2AsU8a(msg, 256);
}

export const hashFuncMap = {
    "Keccak256" : hashKeccak256,
    "Sha2_256" : hashSha2_256,
    "Blake2_64" : hashBlake2_64,
    "Blake2_128" : hashBlake2_128,
    "Blake2_256" : hashBlake2_256
};

export const encodePolkadotAddress = encodeAddress;
