//! TODO - Implement Auction Runtime API
//! 
//! This module defines the Runtime API for getting auction information.
//! 
//! ## Implementation Guide
//! 
//! You need to implement a simple Runtime API that provides auction information.
//! 
//! ### 1. Runtime API Trait
//! Define a Rust trait in your runtime that specifies the auction info function:
//! ```rust
//! #[runtime_interface]
//! pub trait AuctionApi {
//!     /// Get auction information by auction ID
//!     fn get_auction_info(auction_id: AuctionId) -> Option<AuctionInfo>;
//! }
//! ```
//! 
//! ### 2. Required Types
//! Define the necessary types in your runtime:
//! ```rust
//! pub type AuctionId = u32;
//! 
//! #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
//! pub struct AuctionInfo {
//!     pub id: AuctionId,
//!     ...
//! }

//! ```
//! 
//! ### 3. Runtime Implementation
//! Implement the trait in your runtime:
//! ```rust
//! sp_api::decl_runtime_apis! {
//!     impl AuctionApi<Block> for Runtime {
//!         fn get_auction_info(auction_id: AuctionId) -> Option<AuctionInfo> {
//!             AuctionPallet::get_auction_info(auction_id)
//!         }
//!     }
//! }
//! ```
//! 
//! ## References
//! - [KILT Staking Runtime API](https://github.com/KILTprotocol/kilt-node/blob/develop/runtime-api/staking/src/lib.rs)


#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode, MaxEncodedLen};
use frame::deps::sp_api;
use scale_info::TypeInfo;


pub type AuctionId = u32;


/// TODO: Complete auction information structure
/// 
/// Add the following fields from challenge 2:
/// - bid: Option<(AccountId, Balance)> - Current highest bid with bidder
/// - start: BlockNumber - Block when auction starts  
/// - end: Option<BlockNumber> - Block when auction ends
#[derive(Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq, Debug)]
pub struct AuctionInfo {
    /// TODO: Add auction ID field
    pub id: AuctionId,
    
    // TODO: Add current bid field (bidder and amount)
    // pub bid: Option<(AccountId, Balance)>,
    
    // TODO: Add start block field
    // pub start: BlockNumber,
    
    // TODO: Add end block field  
    // pub end: Option<BlockNumber>,
}

// TODO - Define Auction Runtime API
// ============================================================================
// Runtime API Definition
// ============================================================================

sp_api::decl_runtime_apis! {
    pub trait AuctionApi<AccountId, Balance>
    where
        AuctionId: Codec,
    {
        fn get_auction_info(auction_id: AuctionId) -> Option<AuctionInfo>;
    }
}