### Verification threshold

**Carefully read the [Preparing Work](./README.md) first before doing the next.**  

Set contract SQoS type as `Threshold`,  the value is 80(it means only need 80% routers), the value type is `u8`,  and converted to to bytes value is `0x50` , as Fig.3-1 shown. 

![img](../assets/3-1.png)
<p align="center">Fig.3-1 change to Threshold SQoS</p>

Send a greeting message from NEAR testnet.

```sh
1、export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9
​
2、​near call $greeting send_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOUR_NEAR_TEST_ACCOUNT
```

![img](../assets/3-2.png)
<p align="center">Fig.3-2 send a greeting message from NEAR</p>

At present, the number of routers is 3 in POLKADOT testnet, and the Threshold SQoS only needs 80% of routers to process messages, which is 2 routers to receive messages, as shown in Fig.3-3.

![img](../assets/3-3-1.png)
![img](../assets/3-3-2.png)
<p align="center">Fig.3-3 2 routers recevied message</p>