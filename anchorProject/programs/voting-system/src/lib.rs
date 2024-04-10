use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;




declare_id!("qdgqhajmT3X3eckTpbXJxTY2aG2ykERjkwoRejZ7hZF");

#[program]
pub mod voting_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_value : u128) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;  
        vote_account.crunchy = init_value;
        vote_account.smooth = init_value;
        vote_account.admin = *ctx.accounts.user.key;
        Ok(())
    }
    /// Allow account validation logic is handled below at the #[account(...)] macros, letting us just focus on the business logic
    pub fn vote_crunchy(ctx: Context<VoteCrunchy>,vote_value:u128) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.crunchy = vote_value;
        Ok(())
    }
    pub fn vote_smooth(ctx: Context<VoteSmooth>,vote_value:u128) -> ProgramResult {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.smooth = vote_value;
        Ok(())
    }

    pub fn publish_result(ctx: Context<PublishResult>, crunchyresult: u128, smoothresult: u128) -> ProgramResult {

        let vote_account = &mut ctx.accounts.vote_account;
        let user = &mut ctx.accounts.user;
        if vote_account.admin != *user.key {
            return Err(ProgramError::IncorrectProgramId)
        }
        vote_account.smooth = smoothresult;
        vote_account.crunchy = crunchyresult;
        Ok(())
    }
}

/// The #[derive(Accounts)] macro specifies all the accounts that are required for a given instruction
/// Here, we define two structs: Initialize and Vote
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// We mark vote_account with the init attribute, which creates a new account owned by the program
    /// When using init, we must also provide:
    /// payer, which funds the account creation
    /// space, which defines how large the account should be
    /// and the system_program which is required by the runtime

    /// This enforces that our vote_account be owned by the currently executing program, and that it should be deserialized to the VoteAccount struct below at #[account]
    #[account(init, payer = user, space = 264)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct VoteCrunchy<'info> {
    /// Marking accounts as mut persists any changes made upon exiting the program, allowing our votes to be recorded
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[derive(Accounts)]
pub struct VoteSmooth<'info> {
    /// Marking accounts as mut persists any changes made upon exiting the program, allowing our votes to be recorded
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[derive(Accounts)]
pub struct PublishResult<'info> {
    /// Marking accounts as mut persists any changes made upon exiting the program, allowing our votes to be recorded
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user : Signer<'info>
}

/// Here we define what our VoteAccount looks like
/// We define a struct with two public properties: crunchy and smooth
/// These properties will keep track of their respective votes as unsigned 64-bit integers
/// This VoteAccount will be passed inside each Transaction Instruction to record votes as they occur
#[account]
pub struct VoteAccount {
    pub crunchy: u128,
    pub smooth: u128,
    pub admin: Pubkey,
}
