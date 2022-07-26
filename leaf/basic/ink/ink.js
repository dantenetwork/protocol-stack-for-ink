module.exports = {
    // sign and send transaction
    async sendTransaction(contract, methodName, sender, arguments) {
        try {
            let value = 0;
            let gasLimit = -1;
            await contract.tx[methodName]({ value, gasLimit }, ...arguments).signAndSend(sender, (result) => {
                if (result.status.isInBlock) {
                    console.log('in a block');
                } else if (result.status.isFinalized) {
                    console.log('finalized');
                }
            });

            return 'Ok';
        } catch (e) {
        console.error(e);
        }
    },

    // query info from blockchain node
    async contractCall(contract, method, from, arguments) {
        let value = 0;
        let gasLimit = -1;
        const { gasConsumed, result, output } = await contract.query[method](from, {value, gasLimit}, ...arguments);
        return output;
    }
  }