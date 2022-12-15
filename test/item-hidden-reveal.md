### Reveal SQoS

**Carefully read the [Preparing Work](./README.md) first before doing the next.**  

Change contract SQoS type as `Reveal`, no need value, as Fig.2-1 shown. `change_sqos` is only for testing.

![img](../assets/2-1.png)
<p align="center">Fig. 2-1 set Reveal SQoS</p>

Send greeting message from NEAR testnet

```sh
1、export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9
​​
2、near call $greeting send_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOU_NEAR_TEST_ACCOUNT
```

![img](../assets/2-2.png)
<p align="center">Fig. 2-2 send greeting</p>

All routers push hidden message to cross chain

![img](../assets/2-3-1.png)
![img](../assets/2-3-2.png)
<p align="center">Fig.2-3 all routers submitted hidden message</p>

When all routers have completed submitting hidden message, they can continue to submit revealed hidden messages to prevent other routers from copying messages directly.

![img](../assets/2-4-1.png)
![img](../assets/2-4-2.png)
<p align="center">Fig.2-4 all routers revealed message</p>