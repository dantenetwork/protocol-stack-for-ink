import {encodeAddress, blake2AsU8a} from '@polkadot/util-crypto';

export async function polkadotAddressFromPubKey(pk) {
    let compressed = pk;
    
    if ('0x' == pk.substring(0, 2)) {
        compressed = pk.substring(2);
    }

    if (compressed.length == 130) {
        compressed = compressed.substring(2);
    }

    if (compressed.length == 128) {
        const y = "0x" + compressed.substring(64);
        const _1n = BigInt(1);
        let flag = BigInt(y) & _1n ? '03' : '02';
        compressed = flag+compressed.substring(0, 64);
    }

    if (compressed.length == 66) {
        return encodeAddress(blake2AsU8a('0x'+compressed, 256));
    } else {
        throw("`pk` needs to be hex string with length: 66(no 0x), 68(with 0x), 128(no 0x), 130, 132(with 0x), but got " + pk.length);
    }
}
