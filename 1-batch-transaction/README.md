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

## 📤 How to Submit

You'll need to create a Pull Request (PR) that includes two things:

1. **Link to the code** that has been modified to include the `pallet-utility`
2. **Video recording** that demonstrates the successful execution of the batch transfer using the Polkadot-JS Apps UI

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


## 🔗 Basic step to integrate `pallet-utility` to runtime in real application

- [Step 1 - Import pallet into runtime/Cargo.toml](https://github.com/peaqnetwork/peaq-network-node/blob/dev/runtime/peaq/Cargo.toml#L61)
- [Step 2 - Add feauture std](https://github.com/peaqnetwork/peaq-network-node/blob/dev/runtime/peaq/Cargo.toml#L224)
- [Step 3 - Implement Config for Runtime](https://github.com/peaqnetwork/peaq-network-node/blob/dev/runtime/peaq/src/lib.rs#L572)


