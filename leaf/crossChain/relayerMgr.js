'use strict';
const config = require('config');
const relayer = require('./relayer.js');

class relayerMgr {
    constructor() {
        this.relayers = {};
    }

    async init() {
        logger.info("Init relayerMgr");
        let networks = config.get('networks');
        for (let i in networks) {
            let inst = new relayer(i);
            this.relayers[i] = inst;
            await inst.init();
            await utils.sleep(1);
        }
    }
}

let mgr = new relayerMgr();
module.exports = mgr;