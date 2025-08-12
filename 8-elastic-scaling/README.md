# Enable Elastic Scaling Challenge

- **Estimated Time:** 5 days
- **Requirements:**
    1. **Enable Asynchronous Backing:** Elastic scaling is an extension of asynchronous backing. You must first ensure your parachain has asynchronous backing enabled. This involves configuring the `pallet_aura` to allow multiple blocks per slot and adjusting the `MILLIS_PER_BLOCK` to match the relay chain's slot time (typically 6 seconds).
    2. **Configure Runtime Parameters:** Modify your parachain's `runtime/src/lib.rs` to support elastic scaling. This involves:
        - Increasing the `MAX_BLOCK_PROCESSING_VELOCITY` constant to a value greater than `1` (e.g., `3`). This parameter dictates the maximum number of parachain blocks that can be processed per relay chain block.
        - Setting the `UNINCLUDED_SEGMENT_CAPACITY` constant to a value greater than `2 * MAX_BLOCK_PROCESSING_VELOCITY + 1`. This parameter helps manage the queue of unincluded blocks.
    3. **Start a Relay Chain:** You must start a local relay chain (e.g., using `polkadot-launch` or `zombienet`) that is compatible with elastic scaling.
    4. **Start a Parachain:** You must start your modified parachain node and connect it to the relay chain, demonstrating that it successfully registers and begins producing blocks at an increased velocity.
- Related Lesson:
    
    This challenge is a practical application of the concepts from Lesson 5.3 - "Introduction to Elastic Scaling and how to enable elastic scaling on the runtime". The lesson provides the foundational knowledge, and this challenge allows you to implement those changes.
    
- How to Submit:
    
    You'll need to create a Pull Request (PR) containing a link to your GitHub repository where you have made the necessary code changes in the parachain's runtime to enable elastic scaling.

