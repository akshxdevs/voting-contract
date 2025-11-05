Voting-Contract
A Solana Anchor-based program implementing a decentralized voting system.
Overview
Voting-Contract is a smart contract (on Solana, using the Anchor framework) that enables trustless on-chain voting for proposals. It allows a creator to initialize a voting poll with multiple options, voters to cast votes atomically, and results to be tallied transparently. The contract ensures one vote per voter via token or account-based uniqueness and prevents manipulation through timed expiration.
Features

Initialize Poll: The creator sets up a voting poll with a title, description, end timestamp, and list of options.
Vote: Voters select an option; votes are recorded immutably, with checks for prior voting.
Tally Results: After expiration, anyone can query or close the poll to retrieve final vote counts.
Event Emissions: Emits events (InitializeEvent, VoteEvent, CloseEvent) for on-chain monitoring and integration.

How It Works

Initialize: The creator calls the initialize instruction, providing poll details and options. A poll account is created to store state and vote data.
Vote: A voter calls the vote instruction, specifying the poll and their chosen option. The contract verifies the poll is active, the voter hasn't voted, and records the vote.
Close/Tally: After the end time, the close instruction can be called to finalize the poll, emitting results and optionally transferring any rewards.

Main Data Structure
rust#[account]
pub struct Poll {
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub end_time: i64,
    pub options: Vec<String>,
    pub votes: Vec<u64>, // Count per option
    pub voted: Vec<Pubkey>, // List of voters to enforce uniqueness
    pub bump: u8,
}
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

Setup poll by creator calling initialize with options like "Option A", "Option B".
Voters call vote to select an option; votes are appended to the poll.
Post-expiration, call close to tally and emit results.
Query the poll account for real-time vote counts during active period.

Key Files

programs/voting-contract/src/lib.rs: Anchor program logic and instruction definitions.
programs/voting-contract/src/instructions/: Individual instruction handlers (initialize, vote, close).
programs/voting-contract/src/state.rs: Poll account state.
tests/voting-contract.ts: Integration tests covering end-to-end flows.

Events

InitializeEvent: Emitted when a poll is created.
VoteEvent: Emitted when a vote is cast.
CloseEvent: Emitted when the poll is closed and tallied.

Requirements

Node.js, Yarn
Solana CLI tools
Anchor CLI

License
MIT
For more details, see the program code and the test suite.
