const chainHandlerMgr = require('../basic/chainHandlerMgr');
const config = require('config');
const globalDefine = require('../utils/globalDefine.js');
const ErrorCode = globalDefine.ErrorCode;

class Relayer {
  constructor(chainName) {
    this.chainName = chainName;
    this.receiveChains = [];
  }

  async init() {
    let networks = config.get('networks');
    let network = networks[this.chainName];
    this.handler = chainHandlerMgr.getHandlerByName(this.chainName);
    logger.info(
      utils.format(
        'Init relayer: {0}, compatible chain: {1}, receive chains: {2}',
        this.chainName,
        network.compatibleChain,
        network.receiveChains
      )
    );
    this.receiveChains = network.receiveChains;
  }

  async receiveMessageFrom(fromChain) {
    let fromHandler = chainHandlerMgr.getHandlerByName(fromChain);
    let toHandler = this.handler;
    // query sent message count
    const sentMessageCount = await fromHandler.getSentMessageCount(
      this.chainName
    );

    // query next received message Id
    let nextMessageId = await toHandler.getNextMessageId(fromChain);

    if (nextMessageId == 0) {
      logger.info('Not porter, no message can be ported.');
      return;
    }

    logger.info(
      utils.format(
        '{0} <- {1}: {2} has been sent, next received id will be: {3}',
        this.chainName,
        fromChain,
        sentMessageCount,
        nextMessageId
      )
    );

    if (nextMessageId <= sentMessageCount) {
      // get message by id
      const message = await fromHandler.getSentMessageById(
        this.chainName,
        nextMessageId
      );
      logger.debug(
        'Message received, id is ' + nextMessageId + ', raw content: ',
        message
      );

      if (message.errorCode == ErrorCode.GET_MESSAGE_ERROR) {
        logger.warn(
          utils.format(
            'Get sent message from chain {0} error, id is {1}, please check network.',
            fromChain,
            nextMessageId
          )
        );
        return;
      } else if (message.errorCode != ErrorCode.SUCCESS) {
        await toHandler.abandonMessage(
          fromChain,
          nextMessageId,
          message.errorCode
        );
        return;
      }
      let m = message.data;

      let ret = await toHandler.pushMessage(m);

      if (ret != ErrorCode.SUCCESS) {
        await toHandler.abandonMessage(fromChain, nextMessageId, ret);
      }
    }
  }

  async sendMessage() {
    for (let i = 0; i < this.receiveChains.length; i++) {
      await this.receiveMessageFrom(this.receiveChains[i]);
    }
  }

  async executeMessage() {
    let handler = chainHandlerMgr.getHandlerByName(this.chainName);
    let messages = await handler.getExecutableMessage(this.receiveChains);
    for (let i in messages) {
      let fromHandler = chainHandlerMgr.getHandlerByName(messages[i].chainName);
      // const message = await fromHandler.getSentMessageById(this.chainName, nextMessageId);
      const message = await fromHandler.getSentMessageById(
        this.chainName,
        messages[i].id
      );
      let key = {
        chainName: messages[i].chainName,
        id: messages[i].id,
      }
      let completed = await handler.challenge(message.data, key);
      if (completed) {
        await handler.executeMessage(key.chainName, key.id);
      }
    }
  }
}

module.exports = Relayer;
