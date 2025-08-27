# Challenge 4: Auction Forkless Upgrade

## 📋 Overview
Implement auction removal functionality and demonstrate forkless upgrades without network downtime.

## ⏱️ Estimated Time
**2 days**

## 🎯 Requirements

### Prerequisites
Make sure you have finished **Challenge 2: Auction System** before proceeding.

### 1. Implement additional auction removal
The `remove_auction` function allows for the removal of a specific auction from storage. This is typically used to clean up auctions that have concluded or been canceled.


## Hint 
- `TODO - Implement remove_auction extrinsic - make sure that you are finished challenge 2 and 3`
- `TODO - Increase the spec version to 2 ` 

## 📤 How to Submit

You'll need to create a Pull Request (PR) containing two key items:

1. **Link to the code:** A link to the GitHub repository where you have implemented the new Runtime API
2. **Video recording:** A video that demonstrates the following:
   - Showing how to do fork-less upgrade runtime
   - Performing the `remove_auction` on Polkadot JS Explorer

> **Note:** You need to import Alice's account aka Sudo key to Polkadot JS's account or wallet extension like Subwallet, PolkadotJS Extension 
> **Note:** Alice's Private Key: 0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
> **Note:** Do not use Alice's Private Key on Production


## How to run parachain template

For detailed setup instructions, see [Challenge 1: Batch Transactions](../1-batch-transaction/README.md#how-to-run).

## 📚 Related Lessons

This challenge is directly related to:
- **Lesson 4.2** - "Bump a Polkadot SDK codebase"
- **Lesson 3.6** - "Substrate Storage Abstractions & Common Storage"

## 🔗 Resources

- [Runtime Upgrade Tutorial](https://docs.polkadot.com/tutorials/polkadot-sdk/parachains/zero-to-hero/runtime-upgrade/)
- [How to forkless upgrade runtime](https://www.youtube.com/watch?v=k9TgTuRRTZ0)

> **Note:** If you dont want to use this template , you can use `POP` instead
- [Pop](https://learn.onpop.io/chains/guides/create-a-new-parachain) 
