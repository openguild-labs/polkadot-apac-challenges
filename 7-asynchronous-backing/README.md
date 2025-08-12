# Enable Asynchronous Backing Challenge

- **Estimated Time:** 4 days.
- **Requirements:**
    1. **Start a default parachain template:** You must first set up a standard parachain with a **12-second block time target**. This serves as the baseline for your setup.
    2. **Start a relay chain:** Simultaneously, you'll need a relay chain to connect your parachain to.
    3. **Enable Asynchronous Backing:** The core task is to modify the parachain’s configuration to enable asynchronous backing and set its block time target to **6 seconds**.
- Related lessons
    
    This challenge is directly tied to Lesson 5.4 - "Introduction to Asynchronous Backing". The lesson likely covers the theoretical concepts of asynchronous backing, while this challenge provides a hands-on opportunity to implement and test it.
    
- **Resources:**
    - https://github.com/moonbeam-foundation/moonbeam/pull/2776
- **How to submit**
    
    You'll need to create a Pull Request (PR) that includes two things:
    
    1. A link to the code that showing the code changes that enable asynchronous backing
    2. Provide a clear and concise video demonstrating the successful launch  parachain with the configured block times, highlighting that asynchronous backing is active.

