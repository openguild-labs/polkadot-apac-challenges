

### **Challenge 6: The XCM Transfer - Relay Chain to Parachain**

#### 📋 **Overview**

Create a simple dApp that facilitates the transfer of the native Relay Chain asset (PAS) from the **Paseo testnet** to a user's account on the **Asset Hub**, **People Chain**, and others parachain like **Hydration**, **HyperBridge**. This is the most fundamental XCM asset transfer.

#### ⏱️ **Estimated Time**

**2 days**

#### 🎯 **Requirements**

**1. Develop a UI**
Create a user interface that connects to the **Paseo testnet RPC**. The UI must include:

  * An input field for the destination account address (this will be a Polkadot address).
  * An input field for the amount of PAS to transfer.
  * A "Transfer" button to initiate the process.

**2. XCM Message Construction**
The dApp must construct an XCM message using the `xcmPallet.limitedReserveTransferAssets` extrinsic (or a similar reserve-backed transfer). The message must correctly specify:

  * **`dest`**: The destination chain, which is **Asset Hub** or **People Chain** or other parachains. This will be a `MultiLocation` pointing to the parachain.
  * **`beneficiary`**: The recipient's account on Asset Hub. This will be a `MultiLocation` containing the user's `AccountId32`.
  * **`assets`**: The asset being transferred. This will be a `MultiAsset` representing the native PAS token from the Relay Chain (`Parents: 0`).
  * **`feeAssetItem`**: The index of the asset in the `assets` list to be used for paying fees on the destination chain.

**3. On-chain Transaction**
The UI will submit this XCM message as an extrinsic on the **Paseo Relay Chain**. The user signs and sends the transaction from their Paseo account, which must have enough PAS to cover the transfer amount and the transaction fees.

**4. Verification**
After the transaction is finalized on Paseo, the UI must:

  * Query the user's PAS balance on **Parachain**.
  * Display the updated balance, proving the XCM transfer was successful.

#### 📤 **How to Submit**

You'll need to create a Pull Request (PR) containing:

1.  **Link to the code:** A link to the GitHub repository for your dApp.
2.  **Link to the deployed dApp:** A live version of the dApp (e.g., on Vercel or Netlify).
3.  **Video recording:** A video demonstrating:
      * Connecting a wallet with a PAS balance on Paseo.
      * Entering a destination address and amount.
      * Submitting the transaction on Paseo.
      * Showing the PAS balance appear in the recipient's account on Asset Hub, verifiable through the UI and a block explorer.

#### 📚 **Related Lessons**

  * **Lesson 4.7** - "XCM Fundamentals with Locations & Assets": Essential for correctly defining the `MultiLocation` for Asset Hub and the `MultiAsset` for native PAS.
  * **Lesson 4.8** - "Teleport, Transfers and Transact": This challenge is a direct implementation of a reserve-backed transfer.

-----


#### 🔗 **Resources**

  * [Create dot app](https://github.com/preschian/create-dot-app)
  * [Official XCM Documentation](https://www.google.com/search?q=https://docs.polkadot.network/build/cross-chain-messaging/)
  * [ParityTech XCM Docs on Transfers](https://paritytech.github.io/xcm-docs/journey/transfers/index.html)
  * [How to xcm transfer from Relay Chain to Parachain using Polkadot JS app](https://docs.polkadot.com/tutorials/interoperability/xcm-transfers/from-relaychain-to-parachain/)