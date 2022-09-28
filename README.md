# protocol-stack-for-ink

This it the Dante protocol stack for Polkadot.

Let's start with `ink!`. 

<img width="750" alt="web3 foundation_grants_badge_white" src="https://user-images.githubusercontent.com/83746881/187577457-ecf950c6-dfbf-4169-be2f-f03efbf2b674.png">

## Develop record
### Currently
#### Developing
* The SQoS in ink! smart contract, and the details of the SQoS in this stage are [here](https://github.com/w3f/Grants-Program/blob/master/applications/Dante_Network.md#milestone-2--parallel-router-scheduling-algorithms-sqos-off-chain-routers-sdk-testnet).
    * SQoS of the smart contract part;
    * SQoS of the sdk part;
    * SQoS of the off-chain router part.

#### Finished
* Completed the first [milestone](https://github.com/w3f/Grants-Program/blob/master/applications/Dante_Network.md#milestone-1--service-expression-layer--message-verification--router-credibility-evaluation-algorithms-basic-off-chain-routers-basic-sdk) for the w3f grant
* Development of the stable version of [message protocol](https://github.com/dantenetwork/message-ink/tree/v0.1.0) and [ink sdk](https://github.com/dantenetwork/ink-sdk/tree/v0.1.0). Finish the test on Testnet Rococo contracts and local substrate node.
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

#### Next step
* Build the basic SQoS part of Dante protocol for polkadot.
