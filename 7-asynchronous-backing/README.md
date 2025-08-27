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
The core task is to modify the parachain's configuration to enable asynchronous backing and set its block time target to **6 seconds**.

## 📤 How to Submit

You'll need to create a Pull Request (PR) that includes two things:

1. **Link to the code** showing the code changes that enable asynchronous backing
2. **Video recording** demonstrating the successful launch of the parachain with the configured block times, highlighting that asynchronous backing is active

## 📚 Related Lessons

This challenge is directly tied to **Lesson 5.4 - "Introduction to Asynchronous Backing"**. The lesson likely covers the theoretical concepts of asynchronous backing, while this challenge provides a hands-on opportunity to implement and test it.

## 🔗 Resources

- [Moonbeam Asynchronous Backing Implementation](https://github.com/moonbeam-foundation/moonbeam/pull/2776)