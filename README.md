# protocol-stack-for-ink

This it the Dante protocol stack for Polkadot.

*The newest version has been merged into branch `feature-sqos`.*

Let's start with `ink!`. 

<img width="750" alt="web3 foundation_grants_badge_white" src="https://user-images.githubusercontent.com/83746881/187577457-ecf950c6-dfbf-4169-be2f-f03efbf2b674.png">

Note that the resources related to this branch are as follows:
* [message protocol](https://github.com/dantenetwork/message-ink/tree/v0.2.0)
* [ink! SDK](https://github.com/dantenetwork/ink-sdk/tree/feature-sqos)
* [launch a test router](https://github.com/dantenetwork/ink-test-router)

## Develop record
### Currently
#### Developing
* Preparing for the next step.

#### Finished
* Completed the first [milestone](https://github.com/w3f/Grants-Program/blob/master/applications/Dante_Network.md#milestone-1--service-expression-layer--message-verification--router-credibility-evaluation-algorithms-basic-off-chain-routers-basic-sdk) for the w3f grant
* Development of the stable version of [message protocol](https://github.com/dantenetwork/message-ink/tree/v0.2.0) and [ink sdk](https://github.com/dantenetwork/ink-sdk/tree/feature-sqos). Finish the test on Testnet Rococo contracts and local substrate node.
* Development and test of Basic communication components including:
    * Building example dApps composed of smart contracts deployed on different chains, and related [demos](https://github.com/dantenetwork/cross-chain-demo) here.
* The development of algoritms in ink! smart contract:
    * Message verification;
    * Routers evalustion;
    * Routers selection.
* Simulation and testing of some of the algorithms, and the details are [here]().
    * Routers selection in ink! smart contract.
    * Staking and slashing in ink! smart contract.
    * Message verification in other technology stack.
    * Routers evaluation in other technology stack.
* The SQoS in ink! smart contract, and the details of the SQoS in this stage are [here](https://github.com/w3f/Grants-Program/blob/master/applications/Dante_Network.md#milestone-2--parallel-router-scheduling-algorithms-sqos-off-chain-routers-sdk-testnet). Core SQoS items include: 
    * SQoS item `Challenge`
    * SQoS item `Hidden & Reveal`
    * SQoS item `Error Rollback`


#### Next step
* Omniverse Token and Swap Protocols based on Dante Protocol.
* Advanced security mechanisms.

## Testing
Details can be found [here](./test/README.md).

## Usage
### For developers
For developers, there's no need for them to concern about the details of underlying mechanisms. Just follow the [tutorial of ink! SDK](https://github.com/dantenetwork/ink-sdk/tree/feature-sqos) to build your own cross-chain smart contracts.

### For test-router operators
For node operators, try the [tutorial of launching test router](https://github.com/dantenetwork/ink-test-router) for details.