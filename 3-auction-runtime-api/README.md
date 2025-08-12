# Auction Challenges with Runtime APIs  

- **Estimated Time:** 1 day
- **Requirements:**
    1. Make sure you are finished the challenge number 2 
    2. **Define the Runtime API:** You must define a new Rust trait in your runtime that specifies the functions for your API. This trait will include at least one function, such as `get_auction_info(auction_id: AuctionId) -> Option<AuctionInfo>`.
    3. **Implement the Runtime API:** You need to implement this trait for your runtime. The implementation will call the `on-chain` storage getter to retrieve the auction information.
- Related Lesson:
    
    This challenge is directly related to Lesson 3.8 - "Runtime API". The lesson covers how to define and implement a Runtime API to expose custom functionality from your Substrate node to off-chain clients.
    
- How to Submit:
    
    You'll need to create a Pull Request (PR) containing three key items:
    
    1. **Link to the code:** A link to the GitHub repository where you have implemented the new Runtime API
    2. **Video recording:** A video that demonstrates the following:
        - Showing the correct auction information (bidder, amount, end time, etc.) being returned from the API call from Polkadot JS explorer

