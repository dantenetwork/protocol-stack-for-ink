# Challenge SQoS

**Carefully read the [Preparing Work](./README.md) first before doing the next.**  
<br>
The SQoS item `challenge` is easy to understand, and it's a very common mechanism in blockchain field.  

We can call `getEvaluation` of `CROSS CHAIN` contract to get the credibility of the routers, as shown in Fig.1-1.  
Note that `Polkadot.js.app` is not very friendly for `getter` methods, so we cannot include the *interface name* in the *screenshots*.  
![img](../assets/1-1.png)
<p align="center">Fig.1-1 router information</p>

Call `setSqos` of `GREETING` contract to set `Challenge` SQoS, The challenge window period of challenge SQoS we set to 5 minutes(0x0493e0), The value type is `u64`, it needs to be converted to milliseconds and then to bytes, finally value as shown in Fig.1-2. 

![img](../assets/1-2.png)
<p align="center">Fig.1-2 set challenge SQoS</p>

### For normal greeting message

Install near cli `npm install -g near-cli`.

* Send normal greeting message from NEAR testnet. [Prepare a Near Testnet account](https://docs.near.org/concepts/basics/accounts/creating-accounts) before next.

* Export the address of contract `GREETING`, which could be found at [Preparing work](./README.md#polkadot-testnet-contract-address):  
    ```sh
    export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9
    ```
* Use your own near testnet account to send a greeting message to Polkadot:  
    ```sh
    near call $greeting send_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOU_NEAR_TEST_ACCOUNT
    ```
    * The structure of the above command is determined by `Near`, the keys in the map are argument names of a function, so `to_chain`, `title`, `content`, `data` all cannot be changed. 
    * And as `POLKADOTTEST` is the target chain name, it cannot be changed too. 
    * Others(`Greeting`, `Hi there`, `date +'%Y-%m-%d %T'`) are free.
    * `YOU_NEAR_TEST_ACCOUNT` refers to your own account on Near Testnet. 
    * The expected result might be similar as below:  
    ![img](../assets/1-3.png)
    <p align="center">Fig. 1-3 send normal greeting message</p>  
Note the last line in the above picture(Fig. 1-3), the `1`, that's the message number and we can use it to check on the Polkadot side if the message is received.  

* No need for any operations. All honest routers will automatically push the message to the chain, and cross chain contract aggregates messages.  
***The below picture is just the record we have made, and it's printed out by test routers. For testing, there's no need to care about it because Fig.1-5 will prove the result.***

![img](../assets/1-4.png)
<p align="center">Fig. 1-4 router push message to cross chain</p>

* Check the received the message through `CROSS CHAIN` contract on Polkadot side(`getReceivedMessage`):  
![img](../assets/1-5.png)
<p align="center">Fig. 1-5 aggregated messages</p>

For a normal greeting message, no routers will challenge this message during the 5-minutes challenge window. When all routers received and completed the message aggregation, the message will be executed normally.  
* Wait enough time and check the received message through the `GREETING` contract deployed on Polkadot.  
    ![img](../assets/1-6-2.png)
    <p align="center">Fig. 1-6 the greeting message been executed after 5 minutes</p>  

    * The `key` is composed with the information of source chain name(`NEARTEST`) and message id. The message is can be found as we mentioned above in Fig. 1-3.  
    * Another way to get the message id is call `getReceivedMessageNumber` from `CROSS CHAIN` contract, which will return the latest received message number, but it might be not the message id of the sent message as the message is not sure to be arrived when you call or there're other people sent message too.   


## For malicious message
In order to facilitate the testing of challenge SQoS, we provide an interface `submitte_fake_message` to produce a malicious message. 
* We can test it directly through `CROSS CHAIN` contract on Polkadot side, as shown in Fig 1-7-1.
![img](../assets/1-7-1.png)
<p align="center">Fig.1-7-1 submitted a malicious message</p>  

* Check the received message on the `CROSS CHAIN` contract on Polkadot.  
![img](../assets/1-7-2.png)

For all honest routers, after they detected that the source chain message is inconsistent with the destination chain message, they submit a challenge message in the destination cross chain. As shown in Fig.1-8, honest routers submitted a challenge to this malicious message.

* *Challenges* are made automatically by off-chain test routers. Wait enough time to check the received *challenge* information test routers submitted(`getSqosMessage()`).  
![img](../assets/1-8-2.png)
<p align="center">Fig.1-8 3 routers challenge the malicious message</p>

After the time(5-minutes in current deployed `GREETING` contract on Polkadot) of the challenge window period passes, any router who invocates `executeMessage` first will execute this message, and the router executes the message will get reward when incentive mechanism is ready.  
* If the challenges succeed, the cross-chain message will be abandoned and the credibility value of the malicious routers will be reduced. 
* If the challenge failed, the cross-chain message will be executed normally. 
* Call `getEvaluation` of `CROSS CHAIN` contract to get the credibility of the routers. 
* As shown in Fig.1-9, the challenges succeed, and the credibility value of the malicious router is reduced. In Fig.1-1 the related credibility is `4,000`.  

![img](../assets/1-9.png)
<p align="center">Fig.1-9 3 reduce the credibility of malicious router</p>

## Links
* [Setup and Unit-Test](./README.md#setup)
* [Environment Preparing](./README.md#test-environment)
* [SQoS Item: *hidden & reveal*](./item-hidden-reveal.md)
* [SOoS Item: *error rollback*](./item-error-rollback.md)
* [SQoS Item: *verification threshold*](./item-threshold.md)
