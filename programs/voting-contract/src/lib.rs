use anchor_lang::prelude::*;
declare_id!("9xA5BMgH8xC6JkEaJShp6Wn1BqrXu4i1ZXHRgfdsy1fg");

#[program]
pub mod voting_contract {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        title: String,
        vote_type: VoteType,
        dead_line: i64,
        max_votes: u64,
    ) -> Result<()> {
        ctx.accounts.proposal.creator = ctx.accounts.user.key();
        ctx.accounts.proposal.title = title;
        ctx.accounts.proposal.vote_type = vote_type.clone();
        ctx.accounts.proposal.max_votes = max_votes;
        ctx.accounts.proposal.dead_line = dead_line;
        ctx.accounts.proposal.bump = ctx.bumps.proposal;
        ctx.accounts.proposal.is_closed = false;
        match vote_type {
            VoteType::YesNo => {
                ctx.accounts.proposal.yes_votes = 0;
                ctx.accounts.proposal.no_votes = 0;
            },
            VoteType::MultipleChoice =>{

            }
        }
        Ok(())
    }
    pub fn cast_vote(ctx: Context<CastVote>, user_vote: String) -> Result<()> {
        let clock = Clock::get()?.unix_timestamp;
        require!(clock <= ctx.accounts.proposal.dead_line, CustomError::VoteCastTimeOut);
        require!(ctx.accounts.vote_maker.bump == 0, CustomError::AlreadyVoted);
        if ctx.accounts.proposal.vote_type == VoteType::YesNo {
            if user_vote.to_lowercase() == "yes" {
                ctx.accounts.proposal.yes_votes += 1;
            } else if user_vote.to_lowercase() == "no" {
                ctx.accounts.proposal.no_votes += 1;
            }
        }
        ctx.accounts.vote_maker.bump = ctx.bumps.vote_maker;
        Ok(())
    }
    pub fn final_result(ctx: Context<FinalResult>) -> Result<()> {
        let clock = Clock::get()?.unix_timestamp;
        let proposal = &mut ctx.accounts.proposal;

        require!(clock > proposal.dead_line, CustomError::VotingIsAlive);
        require!(!proposal.is_closed, CustomError::AlreadyClosed);

        proposal.is_closed = true;

        msg!("Final Voting Result:");
        msg!("Yes Votes: {}", proposal.yes_votes);
        msg!("No Votes: {}", proposal.no_votes);
        msg!("Total Votes: {}", proposal.yes_votes + proposal.no_votes);        
        Ok(())
    }
}
#[derive(AnchorDeserialize,AnchorSerialize,Clone,PartialEq,Eq)]
pub enum VoteType {
    YesNo,
    MultipleChoice
}
#[account]
pub struct Proposal{
    pub creator: Pubkey,
    pub title:String,
    pub vote_type:VoteType,
    pub is_closed:bool,
    pub yes_votes:u64,
    pub no_votes:u64,
    pub dead_line:i64,
    pub max_votes:u64,
    pub bump:u8,
} 

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"proposal", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 64 + 1 + 1 + 8 + 8 + 8 + 8 + 1
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct VoteMaker{
    pub bump: u8,
}
#[derive(Accounts)]
pub struct CastVote<'info>{
    #[account(mut)]
    pub proposal:Account<'info,Proposal>,
    #[account(
        init,
        seeds = [b"voter", proposal.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 1
    )]
    pub vote_maker: Account<'info, VoteMaker>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>,
}
#[derive(Accounts)]
pub struct FinalResult<'info>{
    #[account(mut, has_one = creator)]
    pub proposal:Account<'info,Proposal>,
    pub creator:Signer<'info>
}
#[error_code]
pub enum CustomError {
    #[msg("Vote Casting Time out!")]
    VoteCastTimeOut,
    #[msg("User already Voted")]
    AlreadyVoted,
    #[msg("Voting is still ongoing.")]
    VotingIsAlive,
    #[msg("Proposal already closed.")]
    AlreadyClosed,
}