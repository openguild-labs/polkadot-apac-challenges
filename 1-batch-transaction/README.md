# Challenge 1: Batch Transactions

## 📋 Overview
Learn how to add `pallet-utility` to your runtime and demonstrate batch transfers using Polkadot-JS Apps UI.

## ⏱️ Estimated Time
**2 days**

## 🎯 Requirements

### 1. Add `pallet-utility` to your runtime
This involves modifying your project's `Cargo.toml` file to include the `pallet-utility` dependency and then configuring it in your runtime's `lib.rs` file. You need to enable the `std` feature in `Cargo.toml` for the pallet as well.

### 2. Interact with `pallet-utility`
Use the **Polkadot-JS Apps UI** (the explorer) to create and sign a batch extrinsic. You'll need to construct a series of `balances.transfer` calls and then wrap them in a `utility.batch` call. This will transfer funds to multiple wallets in a single transaction.


## How to run 

### Prerequisites

Completed the Install [Polkadot SDK Dependencies](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/) guide and successfully installed [Rust](https://www.rust-lang.org/) and the required packages to set up your development environment


### Step 1:  Install `polkadot-omni-node`

```sh
cargo install --locked polkadot-omni-node@0.5.0
```

### Step 2:  Install `staging-chain-spec-builder`

```sh
cargo install --locked staging-chain-spec-builder@10.0.0
```

### Step 3:  Build both node & runtime

```sh
cargo build --workspace --release
```

### Step 4: Use chain-spec-builder to generate the chain_spec.json file

```sh
chain-spec-builder create --relay-chain "dev" --para-id 1000 --runtime \
    target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm named-preset development
```


### Step 5: Run Omni Node

Start Omni Node in development mode (sets up block production and finalization based on manual seal,
sealing a new block every 3 seconds), with a minimal template runtime chain spec.

```sh
polkadot-omni-node --chain chain_spec.json --dev
```

## Hint 

- `TODO : Enable pallet-utility in polkadot-sdk`
- `TODO : Add Utility Pallet here with index 51`
- `TODO : Implement pallet_utility::Config for Runtime` 

## 📤 How to Submit

You'll need to create a Pull Request (PR) that includes two things:

1. **Link to the code** that has been modified to include the `pallet-utility`
2. **Video recording** that demonstrates the successful execution of the batch transfer using the Polkadot-JS Apps UI (https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#)

> **Note:** You need to import Alice's account aka Sudo key to Polkadot JS's account or wallet extension like Subwallet, PolkadotJS Extension 
> **Note:** Alice's Private Key: 0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
> **Note:** Do not use Alice's Private Key on Production

## 📚 Related Lessons

This challenge is directly related to:
- **Lesson 3.4** - "Adding a custom logic to a Runtime"
- **Lesson 3.9** - "Interact with Substrate node on PolkadotJS Apps"

The lesson focuses on understanding and implementing custom runtime logic, with the `pallet-utility` batch transfer being a practical application of this concept.

## 🔗 Resources

- [Substrate Utility Pallet Documentation](https://www.youtube.com/watch?v=diMgOaIYo-s)
- [Polkadot.js Apps Tutorial](https://www.youtube.com/watch?v=uMaSEWajHT0&list=PLnhzaKpksqOKiqu9DDjGnmZWB0hYTaOUC&index=11)
- [Batch Transactions Guide](https://www.youtube.com/watch?v=uoUC2K8muvw)
- [How to add pallet to runtime](https://docs.polkadot.com/develop/parachains/customize-parachain/add-existing-pallets/)
- [Pallet Utility](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/utility)
- [Subwallet](https://www.subwallet.app/download.html)

## 🔗 Basic step to integrate `pallet-utility` to runtime in real application

- [Step 1 - Import pallet into runtime/Cargo.toml](https://github.com/peaqnetwork/peaq-network-node/blob/dev/runtime/peaq/Cargo.toml#L61)
- [Step 2 - Add feauture std](https://github.com/peaqnetwork/peaq-network-node/blob/dev/runtime/peaq/Cargo.toml#L224)
- [Step 3 - Implement Config for Runtime](https://github.com/peaqnetwork/peaq-network-node/blob/dev/runtime/peaq/src/lib.rs#L572)


