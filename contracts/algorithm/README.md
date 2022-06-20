## Introduction

To make users more intuitively to understand how the underly algorithms work, we provide this on-chain prototype smart contract to show the effects of the underlying mechanisms implemented in ink!.

## Index
* [Router selection mechanism](#router-selection-mechanism)
* [Message verification mechanism](#message-verification)

## Currently

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
* Or use the deployed `algorithm_prototype.contract` on the Testnet of AStar, that is `Shibuya Testnet`. The address is *`aX8aZK9Pt9HTgywWYBdskDSdJ9yq6TzLJMDah7vZxfBsYko`*.

##### Operation
* Call `randomRegisterRouters` to register simulation off-chain routers. To make this test simple, you can register enough at a time with any credibilities you want in the parameter vector. The id of the routers will be dynamically created. The registered routers can be checked by calling `getRegisteredRouters `. The result will be something like this:
![1655712763672](https://user-images.githubusercontent.com/83746881/174556149-c6ed625d-b3fa-49fa-b914-bc7b2642a9c9.png)

* Call `selectionTest` to randomly choose `n` routers according to their credibility. Note that the result will be the same if operates in the same block. The result will be like this:
![1655713370200](https://user-images.githubusercontent.com/83746881/174558088-f964c9a1-fa5b-4afb-adf3-a3539f667f61.png)

* Call `selection_statistic` to provide an intuitive validation of the 'Probability distribution' results of the router selection algorithm. Parameter `n` is the number of sampling times. The result will be like this:
![1655713142879](https://user-images.githubusercontent.com/83746881/174557329-c6015cb6-3018-4b24-bdd0-cc830a4746eb.png)

### Message verification


#### Usage
