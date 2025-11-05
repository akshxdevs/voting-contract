# Voting-Contract
**A Solana Anchor-based program implementing a decentralized voting system.**

---

## Overview
`Voting-Contract` is a smart contract (on Solana, using the **Anchor framework**) that enables **trustless on-chain voting** for proposals. It allows a creator to initialize a voting poll with multiple options, voters to cast votes atomically, and results to be tallied transparently.

The contract ensures **one vote per voter** via account-based uniqueness and prevents manipulation through **timed expiration**.

---

## Features
- **Initialize Poll**  
  Creator sets up a poll with title, description, end timestamp, and list of options.
- **Vote**  
  Voters select an option; votes are recorded immutably with prior-vote checks.
- **Tally Results**  
  After expiration, anyone can close the poll to retrieve final vote counts.
- **Event Emissions**  
  Emits `InitializeEvent`, `VoteEvent`, `CloseEvent` for on-chain monitoring.

---

## How It Works

1. **Initialize**  
   Creator calls `initialize`, providing poll details and options. A poll PDA account is created.

2. **Vote**  
   Voter calls `vote`, specifying poll and option index. Contract checks:
   - Poll is active
   - Voter hasn't voted
   - Records vote and marks voter

3. **Close/Tally**  
   After `end_time`, anyone can call `close` to finalize and emit results.

### Main Data Structure
```rust
#[account]
pub struct Poll {
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub end_time: i64,
    pub options: Vec<String>,
    pub votes: Vec<u64>, // Count per option
    pub voted: Vec<Pubkey>, // Enforce one vote per wallet
    pub bump: u8,
}
```

**Events**

| Event              | Description |
|--------------------|-------------|
| `InitializeEvent`  | Emitted on poll creation<br>`creator`, `title`, `end_time`, `option_count` |
| `VoteEvent`        | Emitted on each vote<br>`voter`, `option_index` |
| `CloseEvent`       | Emitted on poll close<br>`final_votes: Vec<u64>` |

**Requirements**
----------------

*   **Node.js ≥ 18**
    
*   **Yarn**
    
*   **Solana CLI**
    
*   avm install latest && avm use latest
    

**License**
-----------

**MIT**
