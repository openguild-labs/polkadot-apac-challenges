# Batch Transactions  Challenges

- **Estimated Time:** 2 day.
- **Requirements:**
    1. **Add `pallet-utility` to your runtime:** This involves modifying your project's `Cargo.toml` file to include the `pallet-utility` dependency and then configuring it in your runtime's `lib.rs` file. You need to enable the `std` feature in `Cargo.toml` for the pallet as well.
    2. **Interact with `pallet-utility`:** Use the **Polkadot-JS Apps UI** (the explorer) to create and sign a batch extrinsic. You'll need to construct a series of `balances.transfer` calls and then wrap them in a `utility.batch` call. This will transfer funds to multiple wallets in a single transaction.
- **How to Submit:**
You'll need to create a Pull Request (PR) that includes two things:
    1. A link to the code that has been modified to include the `pallet-utility`.
    2. A video recording that demonstrates the successful execution of the batch transfer using the Polkadot-JS Apps UI.
- **Related lesson**
This challenge is directly related to **Lesson 3.4 - "Adding a custom logic to a Runtime"**  and Lesson 3.9 - **Interact with Substrate node on PolkadotJS Apps** within a Polkadot SDK bootcamp curriculum. The lesson focuses on understanding and implementing custom runtime logic, with the `pallet-utility` batch transfer being a practical application of this concept.

