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

Usage
Clone the Repo

bashgit clone https://github.com/akshxdevs/voting-contract.git
cd voting-contract
Install Dependencies
bashyarn install
Build the Project
bashanchor build
Test the Project
bashanchor test

Example Flow

Creator calls initialize → poll with "Yes", "No", "Abstain"
Voters call vote(0) → increments count for first option
Real-time tracking via poll account
After deadline → close() emits final tally


Live vote counting during active period
Immutable & verifiable results on-chain


Key Files

programs/voting-contract/src/lib.rs
Anchor program entrypoint and instruction routing
programs/voting-contract/src/instructions/

initialize.rs
vote.rs
close.rs


programs/voting-contract/src/state.rs
Poll struct and account definitions
tests/voting-contract.ts
Full integration tests with mock voters and edge cases


Events





















EventDescriptionInitializeEventEmitted on poll creation (creator, title, end_time)VoteEventEmitted on each vote (voter, option_index)CloseEventEmitted on poll close (final vote counts)

Requirements

Node.js ≥ 18
Yarn
Solana CLI
Anchor CLI
bashavm install latest && avm use latest



License
MIT


GitHub automatically detects and renders any file named README.md at the root of a repository.
This file uses standard Markdown syntax (.md) and will be rendered with:

Headings (#, ##)
Code blocks (bash ... )
Tables
Bold / Italic
Lists
Blockquotes
