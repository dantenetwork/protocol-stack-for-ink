### Verification threshold

**Carefully read the [Preparing Work](./README.md) first before doing the next.**  
<br>
The SQoS item `Threshold` is easy to understand.  

Call `setSqos` of `GREETING` contract to set `Threshold` SQoS, the value is 80(it means only need 80% routers), the value type is `u8`,  and converted to to bytes value is `0x50` , as Fig.3-1 shown. 

![img](../assets/3-1.png)
<p align="center">Fig.3-1 change to Threshold SQoS</p>

* Send normal greeting message from NEAR testnet. [Prepare a Near Testnet account](https://docs.near.org/concepts/basics/accounts/creating-accounts) before next.

* Export the address of contract `GREETING`, which could be found at [Preparing work](./README.md#polkadot-testnet-contract-address):  
    ```sh
    export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9
    ```
* Use your own near testnet account to send a greeting message to Polkadot:  
    ```sh
    ​near call $greeting send_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOU_NEAR_TEST_ACCOUNT
    ```

* Similar result might be found as below:  
![img](../assets/3-2.png)
<p align="center">Fig.3-2 send a greeting message from NEAR</p>

* When we are testing, the number of routers is 3, and the Threshold SQoS only needs 80% of routers to process messages, that is any 2 routers is enough(the contract adopt the first 2 submissions), as shown in Fig.3-3.
    * Check who submitt first by calling `getCurrentRouters/getSelected` of `CROSS CHAIN` contract:  
![img](../assets/3-3-1.png)

* Check the received message by calling `getReceivedMessage` of `CROSS CHAIN` contract:  
![img](../assets/3-3-2.png)
<p align="center">Fig.3-3 2 routers recevied message</p>

## Links
* [Setup and Unit-Test](./README.md#setup)
* [Environment Preparing](./README.md#test-environment)
* [SQoS Item: *challenge*](./item-challenge.md)
* [SQoS Item: *hidden & reveal*](./item-hidden-reveal.md)
* [SOoS Item: *error rollback*](./item-error-rollback.md)
