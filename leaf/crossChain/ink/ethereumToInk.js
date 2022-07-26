'use strict';

const chainHandlerMgr = require('../../basic/chainHandlerMgr');
const logger = require('../../utils/logger');

let fromHandler;
let toHandler;

async function sendMessage(fromChain, toChain) {
    fromHandler = chainHandlerMgr.getHandlerByName(fromChain);
    toHandler = chainHandlerMgr.getHandlerByName(toChain);
    // query ethereum message count
    const ethereumSentMessageCount = await fromHandler.querySentMessageCount(toChain);

    // query ink next receive message Id
    let nextMessageId = await toHandler.getMsgPortingTask(fromChain);
    nextMessageId = parseInt(nextMessageId);
    logger.info(utils.format('{0} <- {1}: {2} has been sent, next received id will be: {3}', toChain, fromChain, ethereumSentMessageCount, nextMessageId));

    if (nextMessageId <= ethereumSentMessageCount) {
        // get message by id
        const jsonRet = await fromHandler.getSentMessageById(toChain, nextMessageId);
        logger.debug('sent message', jsonRet);

        if (jsonRet.errorCode != 0) {
            await toHandler.abandonMessage(fromChain, nextMessageId, jsonRet.errorCode);
            return;
        }
        let message = jsonRet.data;
        
        let ret = await toHandler.pushMessage(message);

        if (ret != 0) {
            await toHandler.abandonMessage(fromChain, nextMessageId, ret);
        }
    }
};

module.exports = {
    sendMessage: sendMessage
}