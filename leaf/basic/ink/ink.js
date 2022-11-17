module.exports = {
    // sign and send transaction
    async sendTransaction(contract, methodName, sender, arguments) {
        try {
            let value = 0;
            let gasLimit = 10**11;
            const options = { storageDepositLimit: null, gasLimit: -1 }
            const { gasRequired, storageDeposit, result } = await contract.query[methodName](
                sender.address,
                options,
                ...arguments
              );
            console.log('gasRequired', gasRequired.toString());
            await new Promise((resolve, reject) => {
				contract.tx[methodName]({ value, gasLimit: gasRequired }, ...arguments)
					.signAndSend(sender, ({ status }) => {
                        if (status.isInBlock) {
                            console.log('in a block');
                            resolve(status.isInBlock)
                        } else if (status.isFinalized) {
                            console.log('finalized');
                            resolve(status.isFinalized);
                        }
					})
			})
            // await contract.tx[methodName]({ value, gasLimit: gasRequired }, ...arguments).signAndSend(sender, (result) => {
            //     if (result.status.isInBlock) {
            //         console.log('in a block');
            //     } else if (result.status.isFinalized) {
            //         console.log('finalized');
            //     }
            // });

            return 'Ok';
        } catch (e) {
            console.error(e);
        }
    },

    // query info from blockchain node
    async contractCall(contract, method, from, arguments) {
        let value = 0;
        let gasLimit = 0;
        const { gasConsumed, result, output } = await contract.query[method](from, {value, gasLimit}, ...arguments);
        return output;
    }
  }