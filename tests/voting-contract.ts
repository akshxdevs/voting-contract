import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingContract } from "../target/types/voting_contract";
import { expect } from "chai";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
chai.use(chaiAsPromised);

// Helper to airdrop SOL to a new user
async function airdropSol(provider: anchor.AnchorProvider, publicKey: anchor.web3.PublicKey, amount = 2 * anchor.web3.LAMPORTS_PER_SOL) {
  const sig = await provider.connection.requestAirdrop(publicKey, amount);
  await provider.connection.confirmTransaction(sig);
}

describe("voting-contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.votingContract as Program<VotingContract>;

  let proposalPDA: anchor.web3.PublicKey;
  let voteMarker: anchor.web3.PublicKey;
  let user: anchor.web3.Keypair;

  it("Initializes a proposal", async () => {
    user = anchor.web3.Keypair.generate();
    await airdropSol(provider, user.publicKey);
    const deadline = new anchor.BN(Math.floor(Date.now() / 1000) + 10);
    [proposalPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), user.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .initialize("Test Proposal", { yesNo: {} }, deadline, new anchor.BN(5))
      .accounts({
        proposal: proposalPDA,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    const proposal = await program.account.proposal.fetch(proposalPDA);
    expect(proposal.creator.toBase58()).to.equal(user.publicKey.toBase58());
    expect(proposal.title).to.equal("Test Proposal");
    expect(proposal.yesVotes.toNumber()).to.equal(0);
    expect(proposal.noVotes.toNumber()).to.equal(0);
  });

  it("Casts a vote (yes)", async () => {
    [voteMarker] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("voter"), proposalPDA.toBuffer(), user.publicKey.toBuffer()],
      program.programId
    );
    await program.methods
      .castVote("yes")
      .accounts({
        proposal: proposalPDA,
        voteMaker: voteMarker,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
  });

  it("Prevents double voting", async () => {
    await expect(
      program.methods
        .castVote("no")
        .accounts({
          proposal: proposalPDA,
          voteMaker: voteMarker,
          user: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc()
    ).to.be.rejectedWith(Error, "User already Voted");
  });

  it("Finalizes the result after deadline", async () => {
    await new Promise((r) => setTimeout(r, 15000));
    await program.methods
      .finalResult()
      .accounts({
        proposal: proposalPDA,
        creator: user.publicKey,
      })
      .signers([user])
      .rpc();
    const proposal = await program.account.proposal.fetch(proposalPDA);
    console.log("Total Votes: ", proposal.maxVotes.toNumber());
    console.log("Yes Votes: ", proposal.yesVotes.toNumber());
    console.log("No Votes: ", proposal.noVotes.toNumber());
    console.log("Is Closed: ", proposal.isClosed);
    expect(proposal.isClosed).to.be.true;
  });
});
