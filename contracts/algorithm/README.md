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
* You can launch a [local substrate node](https://github.com/paritytech/substrate-contracts-node) and deploy the `algorithm_prototype.contract` on it to try. 
* We have deployed `algorithm_prototype.contract` on the Testnet of AStar, that is `Shibuya`. The address is *`aX8aZK9Pt9HTgywWYBdskDSdJ9yq6TzLJMDah7vZxfBsYko`*.

##### Operation


### Message verification

#### Usage
