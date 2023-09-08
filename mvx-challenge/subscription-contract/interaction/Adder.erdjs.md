# lib

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let erdjs = await require('@elrondnetwork/erdjs');
let { erdSys, wallets: { alice } } = await erdjs.setupInteractive("local-testnet");

let lib = await erdSys.loadWrapper("contracts/examples/lib");

// Deploy the lib contract with an initial value of 42
await lib.sender(alice).gas(20_000_000).call.deploy(42);

// Check that the sum is 42
await lib.query.getSum().then((sum) => sum.toString());

await lib.gas(3_000_000).call.add(30);

// Check that the sum is 72
await lib.query.getSum().then((sum) => sum.toString());

```
