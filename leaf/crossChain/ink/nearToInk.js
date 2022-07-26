'use strict';

const chainHandlerMgr = require('../../basic/chainHandlerMgr');
const utils = require('../../utils/utils');

async function sendMessage(fromChain, toChain) {
  let fromHandler = chainHandlerMgr.getHandlerByName(fromChain);
  let toHandler = chainHandlerMgr.getHandlerByName(toChain);
  // query Near message count
  const nearSentMessageCount = await fromHandler.querySentMessageCount(toChain);

  // query Ethereum next receive message Id
  let nextMessageId = await toHandler.getMsgPortingTask(fromChain);
  nextMessageId = parseInt(nextMessageId);

  logger.info(utils.format('{0} <- {1}: {2} has been sent, next received id will be: {3}', toChain, fromChain, nearSentMessageCount, nextMessageId));

  if (nextMessageId <= nearSentMessageCount) {
    // get message by id
    let message = await fromHandler.getSentMessageById(toChain, nextMessageId);
    message = utils.snakeToCamel(message);
    let ret = await toHandler.pushMessage(message);
    if (ret != 0) {
        await toHandler.abandonMessage(nextMessageId, toChain, ret);
    }
  }
};

module.exports = {
  sendMessage: sendMessage
}