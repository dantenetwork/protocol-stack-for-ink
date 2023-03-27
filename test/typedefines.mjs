import exp from 'constants';
import { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } from 'scale-ts';

export const InkAddressData = Struct({
    ink_address: Vector(u8),
    address_type: u8,
});
  
export const MsgDetail = Enum({
    InkString: str,
    InkU8: u8,
    InkU16: u16,
    InkU32: u32,
    InkU64: u64,
    InkU128: u128,
    InkI8: i8,
    InkI16: i16,
    InkI32: i32,
    InkI64: i64,
    InkI128: i128,
    InkStringArray: Vector(str),
    InkU8Array: Vector(u8),
    InkU16Array: Vector(u16),
    InkU32Array: Vector(u32),
    InkU64Array: Vector(u64),
    InkU128Array: Vector(u128),
    InkI8Array: Vector(i8),
    InkI16Array: Vector(i16),
    InkI32Array: Vector(i32),
    InkI64Array: Vector(i64),
    InkI128Array: Vector(i128),
    InkAddress: InkAddressData,
    // UserData: Bytes,
});

// export const ISQoSType = Enum ({
//     Reveal,
//     Challenge,
//     Threshold,
//     Priority,
//     ExceptionRollback,
//     SelectionDelay,
//     Anonymous,
//     Identity,
//     Isolation,
//     CrossVerify,
//     MAX,
// });

export const MessageItem = Struct({
    n: str,
    tv: MsgDetail
});
  
export const MessagePayload = Struct({
    items: Option(Vector(MessageItem))
});

export const TypeSQoSItem = Struct({
    t: u8,
    v: Vector(u8)
});

export const TypeSession = Struct({
    id: u128,
    session_type: u8,
    callback: Vector(u8),
    commitment: Vector(u8),
    answer: Vector(u8)
});

export const IReceivedMessage = Struct(  {
    id: u128,
    from_chain: str,
    to_chain: str,
    sender: Vector(u8),
    signer: Vector(u8),
    sqos: Vector(TypeSQoSItem),
    contract: Vector(u8, 32),
    action: Vector(u8, 4),
    data: Vector(u8),
    session: TypeSession,
});

export const IVVMessageRecved = Struct( {
    recved_msg: IReceivedMessage,
    signature: Vector(u8, 65),
});

export const MsgType = {
    InkString: 'InkString',
    InkU8: 'InkU8',
    InkU16: 'InkU16',
    InkU32: 'InkU32',
    InkU64: 'InkU64',
    InkU128: 'InkU128',
    InkI8: 'InkI8',
    InkI16: 'InkI16',
    InkI32: 'InkI32',
    InkI64: 'InkI64',
    InkI128: 'InkI128',
    InkStringArray: 'InkStringArray',
    InkU8Array: 'InkU8Array',
    InkU16Array: 'InkU16Array',
    InkU32Array: 'InkU32Array',
    InkU64Array: 'InkU64Array',
    InkU128Array: 'InkU128Array',
    InkI8Array: 'InkI8Array',
    InkI16Array: 'InkI16Array',
    InkI32Array: 'InkI32Array',
    InkI64Array: 'InkI64Array',
    InkI128Array: 'InkI128Array',
    InkAddress: 'InkAddress',
    UserData: 'UserData',
};

export const SQoSType = {
    Reveal: 0,
    Challenge: 1,
    Threshold: 2,
    Priority: 3,
    ExceptionRollback: 4,
    SelectionDelay: 5,
    Anonymous: 6,
    Identity: 7,
    Isolation: 8,
    CrossVerify: 9,
    MAX: 10,
};

export class MsgItem {
    constructor(n, tag, value) {
        this.item = {};
        this.item.n = n;
        this.item.tv = {};
        this.item.tv.tag = tag;
        this.item.tv.value = value;
        // this.item.tv[tag] = value;
    }

    async into_raw_data() {

        return Buffer.from([]);
    }
}

export class MsgPayload {
    constructor() {
        this.payload = {
            items: []
        };
    }

    async addItem(item) {
        this.payload.items.push(item.item);
    }

    async encode() {
        return MessagePayload.enc(this.payload);
    }

    async encode2hexstr() {
        return Buffer.from(MessagePayload.enc(this.payload)).toString('hex');
    }

    into_raw_data() {
        let raw_data = Buffer.from([]);

        for (var idx in this.items) {
            raw_data = Buffer.concat([raw_data, this.items[idx]]);
        }

        return raw_data;
    }
}

export class SQoSItem {
    constructor(t, v) {
        this.item = {};
        this.item.t = t;
        this.item.v = Array.from(v);
    }
}

export function createSession(id, sess_type, callback, commitment, answer) {
    return {
        id: BigInt(id),
        sessionType: sess_type,
        callback: Array.from(callback),
        commitment: Array.from(commitment),
        answer: Array.from(answer)
    };
}

// export function transferRecvedMsg2RawData(recvMsg) {
//     let raw_data = Buffer.from(recvMsg.id.toString(16).padStart(32, 0), 'hex');
//     raw_data = Buffer.concat([raw_data, Buffer.from(recvMsg.from_chain, 'utf8'), Buffer.from(recvMsg.to_chain, 'utf8')]);

//     for (var idx in recvMsg.sqos) {
//         raw_data = Buffer.concat([raw_data, Buffer.from([recvMsg.sqos[idx].t]), Buffer.from(recvMsg.sqos[idx].v)]);
//     }

//     let payload = new MsgPayload();
//     payload.payload = MessagePayload.dec(new Uint8Array(recvMsg.data));

//     raw_data = Buffer.concat([raw_data, Buffer.from(recvMsg.contract), Buffer.from(recvMsg.action), payload.into_raw_data()]);

//     raw_data = Buffer.concat([raw_data, Buffer.from(recvMsg.sender), Buffer.from(recvMsg.signer)]);

//     raw_data = Buffer.concat([raw_data, Buffer.from(recvMsg.session.id.toString(16).padStart(32, 0), 'hex'), Buffer.from([recvMsg.session.sessionType])]);
//     raw_data = Buffer.concat([raw_data, Buffer.from(recvMsg.session.callback), Buffer.from(recvMsg.session.commitment), Buffer.from(recvMsg.session.answer)]);


//     return raw_data.toString('hex');
// }

