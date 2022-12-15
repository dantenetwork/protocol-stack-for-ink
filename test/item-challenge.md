# Challenge SQoS

The default initial credibility of the routers at the time of registration is 4000, only cross chain contract owner can call the interface of registerRouter, as shown in Fig.1-1. 

![img](../assets/8.jpg)
![img](../assets/1-1.png)
<p align="center">Fig.1-1 initial router information</p>

The challenge window period of challenge SQoS we set to 5 minutes, The value type is `u64`, it needs to be converted to milliseconds and then to bytes, finally value as shown in Fig.1-2. 

![img](../assets/1-2.png)
<p align="center">Fig.1-2 set challenge SQoS</p>

### For normal greeting message

Install near cli `npm install -g near-cli`.

Send normal greeting message from NEAR testnet

```sh
1、export greeting=d8ae7a513eeaa36a4c6a42127587dbf0f2adbbda06523c0fba4a16bd275089f9

​2、​near call $greeting send_greeting "{\"to_chain\": \"POLKADOTTEST\", \"title\": \"Greeting\", \"content\": \"Hi there\", \"date\": \"`date +'%Y-%m-%d %T'`\"}" --accountId YOU_NEAR_TEST_ACCOUNT
```

![img](../assets/1-3.png)
<p align="center">Fig. 1-3 send normal greeting message</p>

All honest routers push the message to the chain, and cross chain contract aggregates messages

![img](../assets/1-4.png)
<p align="center">Fig. 1-4 router push message to cross chain</p>

![img](../assets/1-5.png)
<p align="center">Fig. 1-5 aggregated messages</p>

For a normal greeting message, no routers will challenge this message during the 5-minutes challenge window. When all routers received and completed the message aggregation, the message will be executed normally.

![img](../assets/1-6-1.png)
![img](../assets/1-6-2.png)
<p align="center">Fig. 1-6 the greeting message been executed after 5 minutes</p>

## For malicious message
In order to facilitate the testing of challenge SQoS, we provide an interface `submitte_fake_message` to produce a malicious message, as shown in Fig 1-7.

![img](../assets/1-7-1.png)
![img](../assets/1-7-2.png)
<p align="center">Fig.1-7 submitted a malicious message</p>

For all honest routers, after they detected that the source chain message is inconsistent with the destination chain message, they submit a challenge message in the destination cross chain. As shown in Fig.1-8, honest routers submitted a challenge to this malicious message.

![img](../assets/1-8-1.png)
![img](../assets/1-8-2.png)
<p align="center">Fig.1-8 3 routers challenge the malicious message</p>

After the 5-minutes challenge window period is over, the router executes this message. If the challenge is successful, the cross-chain message will be abandoned and the credibility value of the malicious routers will be reduced. If the challenge is failed, the cross-chain message will be executed normally. As shown in Fig.1-9, the message challenge is successful, and the credibility value of the malicious router is reduced.

![img](../assets/1-9.png)
<p align="center">Fig.1-9 3 reduce the credibility of malicious router</p>