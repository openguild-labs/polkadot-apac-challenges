# Challenge 2: Auction System

## 📋 Overview
Build a complete auction system with creation, bidding, and management capabilities using `on_initialize` and `on_finalize` hooks.

## ⏱️ Estimated Time
**3 days**

## 🎯 Requirements

### 1. Create a new auction
Developers must use the `new_auction` function to initiate an auction with a defined start block and an optional end block. The pallet will assign a unique `AuctionId` to the new auction.

### 2. Bid on an auction
The primary user-facing function is the `bid` extrinsic. A developer must demonstrate how a signed user can submit a bid for a specific auction ID. The bid must be for an amount greater than any previous bid and the transaction will be validated by the `AuctionHandler` trait.

### 3. Update an auction
The `update_auction` function is used to change the parameters of an existing auction, such as its end block. This function is essential for dynamically managing the lifecycle of an auction.

### 4. Implement Hooks

#### `on_initialize(now)`
This hook is called at the beginning of every new block. It is used to calculate the weight for the `on_finalize` hook, specifically by iterating over all auctions that are scheduled to end at the current block number.

#### `on_finalize(now)`
This hook runs at the end of a block. Its purpose is to process all auctions that have their `end` block set to the current block number (`now`). It drains the `AuctionEndTime` storage for the current block and calls the `on_auction_ended` function from the `AuctionHandler` for each ended auction. This is where the auction's final result is handled, and the auction data is removed from storage.

### 5. Code Examples

```rust
/// Define Auction info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AuctionInfo<AccountId, Balance, BlockNumber> {
	/// Current bidder and bid price.
	pub bid: Option<(AccountId, Balance)>,
	/// Define which block this auction will be started.
	pub start: BlockNumber,
	/// Define which block this auction will be ended.
	pub end: Option<BlockNumber>,
}

/// Hooks 
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	fn on_initialize(now: BlockNumberFor<T>) -> Weight {
		...
	}

	fn on_finalize(now: BlockNumberFor<T>) {
		// if auction time end -> handle logic
	}
}
```

### 6. Mock Runtime and Unit Tests 

## Hint 

- `TODO: Implement Auction Pallet`
- `TODO: Implement pallet_auction::Config for Runtime `

## How to run parachain template

For detailed setup instructions, see [Challenge 1: Batch Transactions](../1-batch-transaction/README.md#how-to-run).

## 📤 How to Submit

You'll need to create a Pull Request (PR) containing three key items:

1. **Link to the code:** A link to the GitHub repository containing both your modified Substrate runtime with `pallet-auction` integrated
2. **Video recording:** A video that demonstrates the full process:
   - Launching the Substrate node with your custom pallet
   - Creating a new auction on Polkadot JS explorer
   - Placing multiple bids from different accounts to show the logic working on Polkadot JS explorer
   - Waiting for the auction to finalize automatically using the `on_finalize` hook and showing the final state on Polkadot JS explorer

   
> **Note:** You need to import Alice's account aka Sudo key to Polkadot JS's account or wallet extension like Subwallet, PolkadotJS Extension 
> **Note:** Alice's Private Key: 0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
> **Note:** Do not use Alice's Private Key on Production


## 📚 Related Lessons

This challenge is directly related to **Lesson 3.7 - "Hooks"**. The lesson covers how to use `on_initialize` and `on_finalize` to schedule and execute pallet logic at specific block numbers.

## 🔗 Resources

- [Hooks Docs by OpenGuild](https://bootcamp.openguild.wtf/building-a-blockchain-with-polkadot-sdk/polkadot-sdk/substrate/adding-a-custom-logic-to-runtime/hooks)
- [Block Production](https://docs.polkadot.com/polkadot-protocol/parachain-basics/blocks-transactions-fees/blocks/#block-production)
- [Official Docs - Parity](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_frame/traits/trait.Hooks.html)
- [Real Integration](https://github.com/UniqueNetwork/unique-chain/blob/develop/pallets/app-promotion/src/lib.rs#L266)

