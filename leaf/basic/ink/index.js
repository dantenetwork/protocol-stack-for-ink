'use strict';

const {ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { Abi, ContractPromise } = require('@polkadot/api-contract');
const { decodeAddress, encodeAddress } = require('@polkadot/keyring');
const { bool, _void, str, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, Enum, Struct, Vector, Option, Bytes } = require('scale-ts');
const utils = require('../../utils/utils');
const { BN } = require("web3-utils");
const config = require('config');
const ink = require('./ink.js');
const fs = require('fs');
const globalDefine = require('../../utils/globalDefine');
const { util } = require('config');
const logger = require('../../utils/logger');

const PLACE_HOLDER = 'X';

const MsgDetail = Enum({
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
  // UserData: Bytes,
});

const SQoSTypeMap = {
  'Reveal': globalDefine.SQoSType.Reveal,
  'Challenge': globalDefine.SQoSType.Challenge,
  'Threshold': globalDefine.SQoSType.Threshold,
  'Priority': globalDefine.SQoSType.Priority,
  'ExceptionRollback': globalDefine.SQoSType.ExceptionRollback,
  'Anonymous': globalDefine.SQoSType.Anonymous,
  'Identity': globalDefine.SQoSType.Identity,
  'Isolation': globalDefine.SQoSType.Isolation,
  'CrossVerify': globalDefine.SQoSType.CrossVerify,
};

const SQoSTypeToInkMap = {
  [globalDefine.SQoSType.Reveal]: 'Reveal',
  [globalDefine.SQoSType.Challenge]: 'Challenge',
  [globalDefine.SQoSType.Threshold]: 'Threshold',
  [globalDefine.SQoSType.Priority]: 'Priority',
  [globalDefine.SQoSType.ExceptionRollback]: 'ExceptionRollback',
  [globalDefine.SQoSType.Anonymous]: 'Anonymous',
  [globalDefine.SQoSType.Identity]: 'Identity',
  [globalDefine.SQoSType.Isolation]: 'Isolation',
  [globalDefine.SQoSType.CrossVerify]: 'CrossVerify',
};

const MsgTypeMap = {
  InkString: globalDefine.MsgType.String,
  InkU8: globalDefine.MsgType.U8,
  InkU16: globalDefine.MsgType.U16,
  InkU32: globalDefine.MsgType.U32,
  InkU64: globalDefine.MsgType.U64,
  InkU128: globalDefine.MsgType.U128,
  InkI8: globalDefine.MsgType.I8,
  InkI16: globalDefine.MsgType.I16,
  InkI32: globalDefine.MsgType.I32,
  InkI64: globalDefine.MsgType.I64,
  InkI128: globalDefine.MsgType.I128,
  InkStringArray: globalDefine.MsgType.StringArray,
  InkU8Array: globalDefine.MsgType.U8Array,
  InkU16Array: globalDefine.MsgType.U16Array,
  InkU32Array: globalDefine.MsgType.U32Array,
  InkU64Array: globalDefine.MsgType.U64Array,
  InkU128Array: globalDefine.MsgType.U128Array,
  InkI8Array: globalDefine.MsgType.I8Array,
  InkI16Array: globalDefine.MsgType.I16Array,
  InkI32Array: globalDefine.MsgType.I32Array,
  InkI64Array: globalDefine.MsgType.I64Array,
  InkI128Array: globalDefine.MsgType.I128Array,
  UserData: globalDefine.MsgType.Bytes,
}

const MsgTypeToInkMap = {
  [globalDefine.MsgType.String]: 'InkString',
  [globalDefine.MsgType.U8]: 'InkU8',
  [globalDefine.MsgType.U16]: 'InkU16',
  [globalDefine.MsgType.U32]: 'InkU32',
  [globalDefine.MsgType.U64]: 'InkU64',
  [globalDefine.MsgType.U128]: 'InkU128',
  [globalDefine.MsgType.I8]: 'InkI8',
  [globalDefine.MsgType.I16]: 'InkI16',
  [globalDefine.MsgType.I32]: 'InkI32',
  [globalDefine.MsgType.I64]: 'InkI64',
  [globalDefine.MsgType.I128]: 'InkI128',
  [globalDefine.MsgType.StringArray]: 'InkStringArray',
  [globalDefine.MsgType.U8Array]: 'InkU8Array',
  [globalDefine.MsgType.U16Array]: 'InkU16Array',
  [globalDefine.MsgType.U32Array]: 'InkU32Array',
  [globalDefine.MsgType.U64Array]: 'InkU64Array',
  [globalDefine.MsgType.U128Array]: 'InkU128Array',
  [globalDefine.MsgType.I8Array]: 'InkI8Array',
  [globalDefine.MsgType.I16Array]: 'InkI16Array',
  [globalDefine.MsgType.I32Array]: 'InkI32Array',
  [globalDefine.MsgType.I64Array]: 'InkI64Array',
  [globalDefine.MsgType.I128Array]: 'InkI128Array',
  [globalDefine.MsgType.Bytes]: 'UserData',
}

const MessageItem = Struct({
  n: str,
  tv: MsgDetail
});

let MessagePayload = Struct({
  items: Option(Vector(MessageItem))
});

const ErrorCode = {
  SUCCESS: 0,
  INTERFACE_ERROR: 1,
  DATA_FORMAT_ERROR: 2,
  ABI_ENCODE_ERROR: 3,
  SEND_TRANSACTION_ERROR: 4,
  GET_TARGET_ERROR: 5,
  DECODE_DATA_ERROR: 6,
  CONVERT_TO_JSON_ERROR: 7,
  MESSAGE_FORMAT_ERROR: 8,
}

class InkHandler {
  constructor(chainName) {
    this.chainName = chainName;
  }

  async init() {
    logger.info(utils.format("Init handler: {0}, compatible chain: {1}", this.chainName, "ink"));
    // network
    this.config = config.get('networks.' + this.chainName);
    this.provider = new WsProvider(this.config.nodeAddress);
    this.api = await ApiPromise.create({provider: this.provider});

    // key
    let secret = JSON.parse(fs.readFileSync(config.get('secret')));
    const keyring = new Keyring({ type: 'sr25519' });
    this.sender = keyring.addFromJson(JSON.parse(secret[this.chainName].backup));
    this.sender.decodePkcs8(secret[this.chainName].password);
    logger.info('Porter address is: ' + this.sender.address);

    // contract
    const crossChainABIRaw = fs.readFileSync(this.config.abiPath);
    this.crossChainContract = new ContractPromise(this.api, JSON.parse(crossChainABIRaw), this.config.crossChainContractAddress);
  }

  // query sent message count
  async querySentMessageCount(chainName) {
    const messageCount =
      await ink.contractCall(this.crossChainContract, 'crossChainBase::getSentMessageNumber', this.sender.address, [chainName]);
    return messageCount.toString();
  }

  // query received message count
  async queryReceivedMessageCount(chainName) {
    const messageCount = await ink.contractCall(
      this.crossChainContract, 'crossChainBase::get_received_message_number', this.sender.address, [chainName]);
    return messageCount.toString();
  }

  // get cross chain message by id
  async getSentMessageById(toChain, id) {
    const crossChainMessage = await ink.contractCall(
      this.crossChainContract, 'crossChainBase::getSentMessage', this.sender.address, [toChain, id]);
    let message = crossChainMessage.asOk.toHuman();
    logger.debug('Original message and data', message, message.content.data);

    // Remove the first byte which is a placeholder
    message.content.contract = message.content.contract.substr(1);
    message.content.action = message.content.action.substr(1);
    
    // sqos
    let sqos = [];
    for (let i = 0; i < message.sqos.length; i++) {
      let item = {};
      item.t = SQoSTypeMap[message.sqos[i].t];
      item.v = message.sqos[i].v;
      sqos.push(item);
    }
    message.sqos = sqos;
    let ret = await this.parseData(message);
    message.content.data = ret.data;
    try {
      utils.checkMessageFormat(message);
    }
    catch (e) {
      logger.error(e);
      return {errorCode: ErrorCode.MESSAGE_FORMAT_ERROR};
    }
    logger.debug('Dealed message', message);
    return {errorCode: ret.errorCode, data: message};
  }

  // get id of message to be ported
  async getMsgPortingTask(chainName) {
    const crossChainMessage = await ink.contractCall(
      this.crossChainContract, 'multiPorters::getMsgPortingTask', this.sender.address, [chainName, this.sender.address]);
    return crossChainMessage;
  }

  // query executable 
  async queryExecutableMessage(chainNames) {
    const _messages = await ink.contractCall(
      this.crossChainContract, 'crossChainBase::getExecutableMessages', this.sender.address, [chainNames]);
    return _messages;
  }

  // push message to cross chain contract
  async pushMessage(message) {
    let dataRet = await this.getEncodedData(message.content.data);
    if (dataRet.errorCode != ErrorCode.SUCCESS) {
      return dataRet.errorCode;
    }
    message.content.data = dataRet.data;

    // prepare message info
    // encode callback
    let callback = null;
    if (message.session.callback) {
      callback = utils.toHexString(utils.stringToByteArray(message.session.callback));
    }

    // deal sqos
    let sqos = [];
    for (let i = 0; i < message.sqos.length; i++) {
      let item = {};
      item.t = SQoSTypeToInkMap[message.sqos[i].t];
      item.v = message.sqos[i].v;
      sqos.push(item);
    }

    let m = {
      id: message.id,
      fromChain: message.fromChain,
      sender: PLACE_HOLDER + message.sender,
      signer: PLACE_HOLDER + message.signer,
      contract: decodeAddress(message.content.contract),
      action: message.content.action,
      data: message.content.data,
      sqos: sqos,
      session: {
        id: message.session.id,
        callback: callback
      }
    }

    // send transaction
    logger.debug('Message to be pushed to chain', m);

    let ret = await ink.sendTransaction(
      this.crossChainContract, 'crossChainBase::receiveMessage', this.sender, [m]);

    if (ret != null) {
      logger.info('Push message successfully, message: ' + m);
      return ErrorCode.SUCCESS;
    }

    return ErrorCode.SEND_TRANSACTION_ERROR;
  }

  // encode the data
  async getEncodedData(data) {
    logger.debug('getEncodedData: ', data);
    let payload = {
      items:[]
    };

    for (let i = 0; i < data.length; i++) {
      let item = {};
      item.n = data[i].name;
      item.tv = {tag: MsgTypeToInkMap[data[i].msgType]};
      let value = data[i].value;
      if (data[i].msgType == globalDefine.MsgType.I128 || data[i].msgType == globalDefine.MsgType.I64 ||
        data[i].msgType == globalDefine.MsgType.U128 || data[i].msgType == globalDefine.MsgType.U64) {
          value = BigInt(value);
      }

      if (data[i].msgType == globalDefine.MsgType.I128Array || data[i].msgType == globalDefine.MsgType.I64Array ||
        data[i].msgType == globalDefine.MsgType.U128Array || data[i].msgType == globalDefine.MsgType.U64Array) {
          for (let j in value) {
            value[j] = BigInt(value[j]);
          }
      }
      item.tv.value = value;
      payload.items.push(item);
    }
    let encoded = utils.toHexString(MessagePayload.enc(payload));
    logger.debug('getEncodedData: ', encoded);

    return {errorCode: ErrorCode.SUCCESS, data: encoded};
  }

  // parse data
  async parseData(message) {
    logger.debug('parseData', message);
    let data = message.content.data;
  
    let payload = MessagePayload.dec(data);
    logger.debug('parseData: decoded data', payload);
    let ret = [];
    if (payload.items) {
      for (let i = 0; i < payload.items.length; i++){
        let item = {};
        item.name = payload.items[i].n;
        item.msgType = MsgTypeMap[payload.items[i].tv.tag];
        item.value = payload.items[i].tv.value;
        
        ret.push(item);
      }
    }

    logger.debug('parseData: result', ret);
    return {errorCode: ErrorCode.SUCCESS, data: ret};
  }

  // execute message
  async executeMessage(chainName, id) {
    // send transaction
    let ret = await ink.sendTransaction(
      this.crossChainContract, 'crossChainBase::executeMessage', this.sender, [chainName, id]);

    if (ret != null) {
      logger.info(
        this.chainName + ' messageId ' + id + ' executed, fromChain ' + chainName);
    }
  }

  // abandon message
  async abandonMessage(chainName, id, errorCode) {
    return;
    // send transaction
    let ret = await ink.sendTransaction(
      this.crossChainContract, 'crossChainBase::abandonMessage', this.sender, [chainName, id, errorCode]);

    if (ret != null) {
      logger.info(utils.format('Abandon message id: {0} successfully, errorCode is: {1}', id, errorCode));
    }
  }

  getProvider() {
    return this.provider;
  }
}

module.exports = InkHandler;