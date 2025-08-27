# Challenge 3: Auction Runtime API

## 📋 Overview
Extend the auction system with custom Runtime API for off-chain data access and real-time auction information retrieval.

## ⏱️ Estimated Time
**2 day**

## 🎯 Requirements

### Prerequisites
Make sure you have finished **Challenge 2: Auction System** before proceeding.

### 1. Define the Runtime API
You must define a new Rust trait in your runtime that specifies the functions for your API. This trait will include at least one function, such as `get_auction_info(auction_id: AuctionId) -> Option<AuctionInfo>`.

### 2. Implement the Runtime API
You need to implement this trait for your runtime. The implementation will call the `on-chain` storage getter to retrieve the auction information.

## 📤 How to Submit

You'll need to create a Pull Request (PR) containing two key items:

1. **Link to the code:** A link to the GitHub repository where you have implemented the new Runtime API
2. **Video recording:** A video that demonstrates the following:
   - Showing the correct auction information (bidder, amount, end time, etc.) being returned from the API call from Polkadot JS explorer

## Hint
- `TODO: Complete auction information structure`
- `TODO - Define Auction Runtime API` 
- `TODO: Implement runtime API for auction pallet`

## How to run parachain template

For detailed setup instructions, see [Challenge 1: Batch Transactions](../1-batch-transaction/README.md#how-to-run).


## 📚 Related Lessons

This challenge is directly related to **Lesson 3.8 - "Runtime API"**. The lesson covers how to define and implement a Runtime API to expose custom functionality from your Substrate node to off-chain clients.


## 🔗 Resources

- [Runtime API Definition](https://docs.polkadot.com/polkadot-protocol/parachain-basics/node-and-runtime/#runtime-apis)
- [Runtime API Videos](https://www.youtube.com/watch?v=BTz39Kzlv-U&list=PLnhzaKpksqOKiqu9DDjGnmZWB0hYTaOUC&index=9)
- [Runtime API OpenGuild](https://bootcamp.openguild.wtf/building-a-blockchain-with-polkadot-sdk/polkadot-sdk/substrate/runtime-api-and-rpc)
- [Real case](https://github.com/KILTprotocol/kilt-node/tree/develop/runtime-api)
