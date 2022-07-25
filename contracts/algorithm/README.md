## Introduction

To make users more intuitively to understand how the underly algorithms work, we provide this on-chain prototype smart contract to show the effects of the underlying mechanisms implemented in ink!.

## Index
* [Router selection mechanism](#router-selection)
* [Message verification mechanism](#message-verification)
* [Router evaluation mechanism](#router-evaluation)

## Demos

### Router selection

#### Effects
A statistic result of this algorithm is as below:

![1655383046092](https://user-images.githubusercontent.com/83746881/174071425-78fbea88-2f20-41c8-b874-fec4d61208c5.png)

* The `Credibility Ratio` is the probability distribution of the routers to be selected as working nodes according to their credibilities. In product implementation, this probability will be calculated considering both credibility(reflecting history work) and staking(reflecting economics incentives).
* The `selected results` is the result by calling `selection_statistic`.

We can see in the picture above, that the theoretical value of probability distribution is nearly the same as the real result of the selection algorithm.

#### Usage
##### Deploy
Try the following operations with [polkadot,js/app](https://polkadot.js.org/apps/#/explorer). On the `Shibuya Testnet` or deploy the smart contract `algorithm_prototype.contract` in *./bin* on a local substrate node.

* You can launch a [local substrate node](https://github.com/paritytech/substrate-contracts-node) and deploy the `algorithm_prototype.contract` on it to try. 
* Or use the deployed `algorithm_prototype.contract` on the Testnet of AStar, that is `Shibuya Testnet`. The address is *`aiNehN39tGVVtJyWCkWNWBEWQJjftLafCUzKVxG9tqvzPGx`*, and the related `metadata.json` is in *./bin*.

##### Operation
* Call `randomRegisterRouters` to register simulation off-chain routers. To make this test simple, you can register enough at a time with any credibilities you want in the parameter vector. The id of the routers will be dynamically created. The registered routers can be checked by calling `getRegisteredRouters `. The result will be something like this:

![1655712763672](https://user-images.githubusercontent.com/83746881/174556149-c6ed625d-b3fa-49fa-b914-bc7b2642a9c9.png)

* Call `selectionTest` to randomly choose `n` routers according to their credibility. Note that the result will be the same if operates in the same block. The result will be like this:

![1655714656855](https://user-images.githubusercontent.com/83746881/174563243-46a3ae26-5fb0-47c7-b7c1-57aee72177cd.png)

* Call `selection_statistic` to provide an intuitive validation of the 'Probability distribution' results of the router selection algorithm. Parameter `n` is the number of sampling times. The result will be like this:

![1655715182550](https://user-images.githubusercontent.com/83746881/174564024-8b11d9c2-b15a-4bb7-9b34-e11971e3fa6a.png)

### Message verification
In message verification algorithm prototype, we descide the limitation number of message copies to verify a message by a system paremeter `self.msg_copy_count`. When enough copies have been delivered, `simu_message_verification` will be called dynamically.

And the number determines how many routers one message needs to be delivered parallelly, which will be configured by users through SQoS settings in the product implementation. 

#### Usage
* Call `setSysinfo` to set message-verification related system paremeters and `getSysinfo` to check the value.
  * `1` is the number of the message copies needed for one message to be verified
  * `2` is the credibility threshold, a factor of 10,000 was multiplied in order to calculate on-chain. `7000` means that one message copy will be accepted only if it has at least 70% of the aggregated weight according to credibilities.

![1655716365972](https://user-images.githubusercontent.com/83746881/174568231-9529d91a-8cd5-4510-b191-64e7e4360462.png)

* Call `simuSubmitMessage` to submite message copies manually. Note that message copies belong to the same message only if they have the same `IReceivedMessage::id` and `IReceivedMessage::from_chain`. 

![1655718496147](https://user-images.githubusercontent.com/83746881/174575620-555750dd-5e84-47fb-8225-9d91c2c20efa.png)

* The submitted message copies can be checked by `simuGetMessage `.




*  When enough message copies are submitted, `simu_message_verification` will be called internally. The result will be cached in order to be checked manually and an event `VerifiedMessage` will be emitted to show the result, but the result event need to be decoded by `Polkadot.js`. 



* Check the cached verification result by `getVerifiedResults`.

![1655721713779](https://user-images.githubusercontent.com/83746881/174584850-ed1cc4d8-42d6-4844-98f0-f4b75b3872b1.png)

There are three contents of the submitted message copies. The first one is submitted by routers `0`, `1`, and `2`. The second one is submitted by router `3`. And the third one is submitted by router `4`. The aggregated credibility weights are `240`, `60`, and `50` respectively. As `240` is only 68%(need 70%) of the total `240 + 60 + 50 = 350`, the verification, in this case, does not pass, so none of these three copies is accepted.

### Router evaluation
In router evaluation algorithm prototype, we provide a direct way to demostrate the how an honest or evil behavior effects the variation of routers' credibility.

#### Usage
* Call `randomRegisterRouters ` to register a router with whatever crediblity you like, or use the registered routers above.
![1658735675933](https://user-images.githubusercontent.com/83746881/180726816-7e66db44-57a1-44fb-9520-8bcac8ba6329.png)

* Call `doHonestOnce ` to simulate a router completing the delivery of one message copy successfully, whose credibility increases then.
![1658735920961](https://user-images.githubusercontent.com/83746881/180727555-b61facee-2326-40cd-8f09-55d35102e309.png)

* Call `doEvilOnce` to simulate a router making malicious things and been caught, whose credibility decreases then.
![1658736068106](https://user-images.githubusercontent.com/83746881/180728105-16dbca64-f388-4f82-88d8-9a87e4b1682f.png)

* Call `getCredibility` to get the related router's credibility.
![1658736154730](https://user-images.githubusercontent.com/83746881/180728339-ee55c687-d523-4323-8749-9e80262b37a0.png)

#### Test scripts
To learn the trend of the node evaluation algorithms clearly, we provide a [test scripts](../../test/nodeEvaluation.js) to show that.
* We call `randomRegisterRouters` throungh `Polkadot.js.app` first to add a new router with credibility score 10.
* In this JS scripts, we call `doHonest` and do honest behavior 20 times once and get an event containing results everytime. We get an increased curve as below:
![Honest](https://user-images.githubusercontent.com/83746881/180753234-bc432d9e-0c4c-4599-90cb-90377be6a6e0.png)  
**Note that the score will not increase as soon as got 90 because of the precision. We use 0~10,000 to describe the credibility in our official version.**

* We call `doEvil` and do malicious behavior 20 times once and get an event containing results everytime. We get a decrease curve as below:
![evil](https://user-images.githubusercontent.com/83746881/180753589-a0eb0548-7144-4ce1-868e-c75e16ce2850.png)  
**Note that the score will not decrease as soon as got 4 because of the precision. We use 0~10,000 to describe the credibility in our official version.**


