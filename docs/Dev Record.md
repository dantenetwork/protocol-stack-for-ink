# Dev record

## Common Abnormalities
* Error `Input too large. Found input with 28 bits, expected 8`. The reason of this error is probably invalid input(parameters), do the following:
    * Check contract address;
    * Check abi file (***.json);
    
## Skills
* Encoding and Decoding skills:
  * off-chain node.js: [code](https://github.com/dantenetwork/protocol-stack-for-ink/blob/07ee6c7d9fc77d603dbdd40e68fa30c69a30ce16/test/app.js#L45)
  * on-chain smart contract: [code](https://github.com/dantenetwork/protocol-stack-for-ink/blob/07ee6c7d9fc77d603dbdd40e68fa30c69a30ce16/contracts/Protocol/lib.rs#L121)

* Advanced usage of `scale_info::TypeInfo` for ABI generation related skills
  * on-chian smart contract: [code](https://github.com/dantenetwork/protocol-stack-for-ink/blob/07ee6c7d9fc77d603dbdd40e68fa30c69a30ce16/contracts/Payload/lib.rs#L23). With out the manual implementation, user-defined struct cannot be used as libs for others.
