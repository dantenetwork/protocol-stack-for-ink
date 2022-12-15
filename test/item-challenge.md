# Challenge SQoS

**Carefully read the [Preparing Work](./README.md) first before doing the next.**  

We can call `getEvaluation` of `CROSS CHAIN` contract to get the credibility of the routers, as shown in Fig.1-1.  
Note that `Polkadot.js.app` is not very friendly for `getter` methods, so we cannot include the *interface name* in the *screenshots*.  
![img](../assets/1-1.png)
<p align="center">Fig.1-1 router information</p>

The challenge window period of challenge SQoS we set to 5 minutes, The value type is `u64`, it needs to be converted to milliseconds and then to bytes, finally value as shown in Fig.1-2. 

![img](../assets/1-2.png)
<p align="center">Fig.1-2 set challenge SQoS</p>

### For normal greeting message

Install near cli `npm install -g near-cli`.

Send normal greeting message from NEAR testnet

* Export the address of contract `GREETING`:  
    ```sh
    export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9
    ```
* Use your own near testnet account to a greeting message to Polkadot:  
    ```sh
    ​near call $greeting send_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOU_NEAR_TEST_ACCOUNT
    ```
    * The structure of the above command is determined by `Near`, the keys in the map are argument names of a function, so `to_chain`, `title`, `content`, `data` all cannot be changed. 
    * And as `POLKADOTTEST` is the target chain name, it cannot be changed too. 
    * Others(`Greeting`, `Hi there`, `date +'%Y-%m-%d %T'`) are free.
    * `YOU_NEAR_TEST_ACCOUNT` refers to your own account on Near Testnet. 
    * The expected result might be similar as below:  
    ![img](../assets/1-3.png)
    <p align="center">Fig. 1-3 send normal greeting message</p>  
Note the last line int the above picture(Fig. 1-3), the `1`, that's the message number and we can use it to check on the Polkadot side if the message is received.  

* All honest routers push the message to the chain, and cross chain contract aggregates messages.  
***Note that this is processed by the test routers automatically, so there's no need for any operation.***

![img](../assets/1-4.png)
<p align="center">Fig. 1-4 router push message to cross chain</p>

* Check the received the message through `CROSS CHAIN` contract on Polkadot side(`getReceivedMessage`):  
![img](../assets/1-5.png)
<p align="center">Fig. 1-5 aggregated messages</p>

For a normal greeting message, no routers will challenge this message during the 5-minutes challenge window. When all routers received and completed the message aggregation, the message will be executed normally.  
* Wait enough time and check the received message through the `GREETING` contract deployed on Polkadot.  
![img](../assets/1-6-2.png)
<p align="center">Fig. 1-6 the greeting message been executed after 5 minutes</p>

## For malicious message
In order to facilitate the testing of challenge SQoS, we provide an interface `submitte_fake_message` to produce a malicious message. 
* We can test it directly through `CROSS CHAIN` contract on Polkadot side, as shown in Fig 1-7-1.
![img](../assets/1-7-1.png)
<p align="center">Fig.1-7-1 submitted a malicious message</p>
* 
![img](../assets/1-7-2.png)


For all honest routers, after they detected that the source chain message is inconsistent with the destination chain message, they submit a challenge message in the destination cross chain. As shown in Fig.1-8, honest routers submitted a challenge to this malicious message.

![img](../assets/1-8-1.png)
![img](../assets/1-8-2.png)
<p align="center">Fig.1-8 3 routers challenge the malicious message</p>

After the 5-minutes challenge window period is over, the router executes this message. If the challenge is successful, the cross-chain message will be abandoned and the credibility value of the malicious routers will be reduced. If the challenge is failed, the cross-chain message will be executed normally. As shown in Fig.1-9, the message challenge is successful, and the credibility value of the malicious router is reduced.

![img](../assets/1-9.png)
<p align="center">Fig.1-9 3 reduce the credibility of malicious router</p>