# Challenge 7: Asynchronous Backing

## 📋 Overview
Enable asynchronous backing on a parachain with 6-second block time, demonstrating improved performance and relay chain integration.

## ⏱️ Estimated Time
**4 days**

## 🎯 Requirements

### 1. Start a default parachain template
You must first set up a standard parachain with a **12-second block time target**. This serves as the baseline for your setup.

### 2. Start a relay chain
Simultaneously, you'll need a relay chain to connect your parachain to.

### 3. Enable Asynchronous Backing

Always add **#[cfg(feature = "async-backing")]** when you add codes to enable the `Async Backing` Feature

For example: 
```rust
#[cfg(feature = "async-backing")]
use crate::{constants::SLOT_DURATION, types::ConsensusHook};
```


The core task is to modify the parachain's configuration to enable asynchronous backing and set its block time target to **6 seconds**.


## How to run 

### Step 1: Set up default toolchain for compatible version 

```
rustup default nightly-2024-06-12
```
### Step 2: Set up Zombienet - Install relaychain 

```bash
./scripts/zombienet.sh build 
```



### Step 3: After enable `Async Backing` Feature , build the project

```bash
cargo build --release --features async-backing
```

### Step 4: Run zombienet 

```bash
./scripts/zombienet.sh devnet
```

## 📤 How to Submit

You'll need to create a Pull Request (PR) that includes two things:

1. **Link to the code** showing the code changes that enable asynchronous backing
2. **Video recording** demonstrating the successful launch of the parachain with the configured block times, highlighting that asynchronous backing is active

## 📚 Related Lessons

This challenge is directly tied to **Lesson 5.4 - "Introduction to Asynchronous Backing"**. The lesson likely covers the theoretical concepts of asynchronous backing, while this challenge provides a hands-on opportunity to implement and test it.

## 🔗 Resources

- [Moonbeam Asynchronous Backing Implementation](https://github.com/moonbeam-foundation/moonbeam/pull/2776)
- [Official Guide](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/async_backing_guide/index.html)
- [Wiki](https://wiki.polkadot.com/learn/learn-async-backing/)
- [Official Async Backing](https://docs.polkadot.com/develop/parachains/maintenance/configure-asynchronous-backing/)