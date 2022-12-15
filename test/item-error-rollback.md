### Error rollback

**Carefully read the [Preparing Work](./README.md) first before doing the next.**  

In order to test `error rollback`, we added the `send_fake_greeting` interface in the greeting contract of NEAR testnet.

```sh
1、export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9
​
2、​near call $greeting send_fake_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOU_NEAR_TEST_ACCOUNT
```

![img](../assets/4-1.png)
<p align="center">Fig.4-1 send fake greeting message</p>

POLKADOT testnet received this fake message, and an error will be made when the cross-chain contract executes this message, and sends an error rollback to NEAR testnet, as shown in Fig.4-2 and Fig.4-3.

![img](../assets/4-1.png)
<p align="center">Fig.4-2 received fake greeting message</p>

![img](../assets/4-3.png)
<p align="center">Fig.4-3 send error rollback</p>

For NEAR testnet it will receive this error rollback, as shown Fig.4-4.

![img](../assets/4-4-1.png)
![img](../assets/4-4-2.png)
<p align="center">Fig.4-4 NEAR testnet receive error</p>