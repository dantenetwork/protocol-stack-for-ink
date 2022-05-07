# protocol-stack-for-ink

This it the first step of Dante protocol stack for Polkadot.

Let's start with `ink!`. 

## Dev record

* Error `Input too large. Found input with 28 bits, expected 8`. The reason of this error is probably invalid input(parameters), do the following:
    * Check contract address;
    * Check abi file (***.json);
