# Challenge 6: NFT XCM

## 📋 Overview
Create a dApp that mints NFTs on Asset Hub using XCM messages from Paseo testnet, demonstrating cross-chain communication and complex message construction.

## ⏱️ Estimated Time
**4 days**

## 🎯 Requirements

### 1. Develop a UI
Create a user interface that connects to the **Paseo testnet RPC**. The UI should have a button or form to initiate the NFT minting process.

### 2. XCM Message Construction
The dApp must construct a complex XCM message with the following instructions:

#### Fund the transaction
The message must first `ReserveAssetDeposited` to reserve assets from the Paseo Relay Chain's sovereign account on Asset Hub. This will be used to pay for the minting transaction fee on Asset Hub. The user must have a balance on Paseo, but not on Asset Hub, so the fees must come from the Paseo account.

#### Execute the minting
The XCM message must include a `Transact` instruction. This instruction will contain a SCALE-encoded call to the `pallet-nfts::mint` extrinsic on **Asset Hub**, specifying the collection ID, item ID, and the recipient account.

### 3. On-chain Demonstration
The UI should then submit this XCM message as an extrinsic on the Paseo network. You must demonstrate that this single transaction from Paseo results in an NFT being minted on Asset Hub.

### 4. Verification
After the transaction, the UI must display the newly minted NFT by querying the Asset Hub's blockchain state, proving the cross-chain operation was successful.

## 📤 How to Submit

You'll need to create a Pull Request (PR) containing three key items:

1. **Link to the code:** A link to the GitHub repository containing the source code for your dApp
2. **Link to the deployed dApp:** A link to a live version of the dApp, which can be deployed on platforms like Vercel or Netlify
3. **Video recording:** A video demonstrating the entire process, including:
   - Connecting a wallet with a balance on Paseo
   - Initiating the minting process via the dApp's UI
   - Showing the transaction being signed and submitted on Paseo
   - Finally, showing the newly minted NFT appear on the Asset Hub block explorer and within the dApp's UI

## 📚 Related Lessons

- **Lesson 4.7** - "XCM Fundamentals with Locations & Assets": This is critical for understanding the `MultiLocation` and `MultiAsset` types needed to correctly address the Asset Hub and the native Paseo asset
- **Lesson 4.8** - "Teleport, Transfers and Transact": This challenge is a direct application of the `Transact` instruction, which allows one chain to tell another to execute a specific extrinsic on its behalf

## 🔗 Resources


- [Polkadot API to interact with runtime pallet](https://papi.how/)
- [@polkadot/api](https://www.npmjs.com/package/@polkadot/api)
- [Create dot app](https://github.com/preschian/create-dot-app)
    