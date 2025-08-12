# Polkadot APAC Bootcamp 2025

</div>

## (Optional) Setup environment and register for the challenges

TLDR: If you are not familiar with Git & Github, follow these steps below to fork and setup your own repository.

- Step 1: Install Git & Github Desktop (optional) on your local device
- Step 2: Fork this repository by click the `Fork button` on Github

![image](https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7fa2f01a-b523-4208-92db-d8af7a274d98)

- Step 3: `Clone` the forked repository to your local device using the below command

```sh
git clone https://github.com/<your_github_username>/polkadot-apac-challenges.git
```

Replace the `[name-of-your-account]` with your Github username. For example, if my username is `chungquantin`, I would do the below command to clone the repository to my local device.

```sh
git clone https://github.com/openguild-labs/polkadot-apac-challenges.git
```

- Step 4: Edit the `README.md` file to register for official participation

Go to **Participant Registration** section and register to be the workshop participants. Add the below to the list, replace any placeholder with your personal information.

```
| 🦄 | Dustin | CocDap | DevRel at OpenGuild |
```

- Step 5: `Commit` your code and push to the forked Github repository

```
git add .
git commit -m "<Your Name> | Register for OpenGuild Da Nang Hackcamp 2025"
```

- Step 6: Create a `Pull Request` to merge your changes to this repository and name your PR as `Your name | Register for Polkadot APAC Challenges 2025`

<img width="1166" alt="Screenshot 2024-04-19 at 16 23 45" src="https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7554ca7d-da68-4a23-893a-4f2c11a78d37">

<br/>

<div align="center">




## Discover the List of Challenges 🏆

| Challenge | Description | Action |
| --------- | ----------- | ------ |
| 1 | Batch Transactions - Add pallet-utility to runtime and demonstrate batch transfers using Polkadot-JS Apps UI | [Take Challenge](./1-batch-transaction/README.md) |
| 2 | Auction System - Create, bid, and manage auctions with on_initialize and on_finalize hooks | [Take Challenge](./2-auction/README.md) |
| 3 | Auction Runtime API - Extend auction system with custom Runtime API for off-chain data access | [Take Challenge](./3-auction-runtime-api/README.md) |
| 4 | Auction Forkless Upgrade - Implement auction removal functionality and demonstrate forkless upgrades | [Take Challenge](./4-auction-forkless-upgrade/README.md) |
| 5 | On-chain Identity DApp - Build a dApp for registering and displaying on-chain identities on Paseo testnet | [Take Challenge](./5-on-chain-identity/README.md) |
| 6 | NFT XCM - Create a dApp that mints NFTs on Asset Hub using XCM messages from Paseo testnet | [Take Challenge](./6-nft-xcm/README.md) |
| 7 | Asynchronous Backing - Enable asynchronous backing on a parachain with 6-second block time | [Take Challenge](./7-asynchronous-backing/README.md) |
| 8 | Elastic Scaling - Configure parachain runtime for elastic scaling with increased block processing velocity | [Take Challenge](./8-elastic-scaling/README.md) |
