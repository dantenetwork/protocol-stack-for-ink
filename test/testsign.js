const { ApiPromise, WsProvider } = require('@polkadot/api');
const { stringToHex } = require('@polkadot/util');
const { web3FromSource } = require('@polkadot/extension-dapp');

// const jsdom = require('jsdom');
// const { JSDOM } = jsdom;

// const dom = new JSDOM(`<!DOCTYPE html><p>Hello world</p>`);
// const window = dom.window;
// const document = window.document;

// Replace this with your own target DOT address
const targetDotAddress = {
  address: '5FgN7rEmkH9XQSNdLxMgFUbYVYFGtrA36HwysM1CrkW8tfGk',
  meta: {
    name: 'My DOT Address',
    source: 'polkadot-js browser extension'
  }
};

async function signMessage() {
  // Create a new API instance using a WebSocket provider
//   const provider = new WsProvider('wss://rpc.polkadot.io');
//   const api = await ApiPromise.create({ provider });

  // Get the signer from the injected provider (e.g. Polkadot.js browser extension)
  const injector = await web3FromSource(targetDotAddress.meta.source);
  const signer = injector?.signer;

  if (!signer) {
    throw new Error('Failed to retrieve signer');
  }

  // Create a new raw message to sign
  const message = 'I\'m verifying my DOT address';
  const data = stringToHex(message);
  const type = 'bytes';

  // Sign the message using the signer
  const signature = await signer.signRaw({ address: targetDotAddress.address, data, type });

  console.log(signature);

  // Verify the signature using the API
//   const isValid = await api.tx.utility.verify({
//     signer: targetDotAddress.address,
//     signature: signature,
//     data: data,
//     era: 0
//   });

//   console.log(`Signature is ${isValid ? 'valid' : 'invalid'}`);
}

signMessage().catch(console.error);
