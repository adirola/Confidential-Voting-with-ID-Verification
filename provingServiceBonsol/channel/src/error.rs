use anagram_bonsol_schema::error;
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ChannelError {
    #[error("Invalid Requester Account")]
    InvalidRequesterAccount,
    #[error("Invalid Execution Account")]
    InvalidExecutionAccount,
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Invalid Input Data")]
    InvalidInputs,
    #[error("Invalid Input Lenght")]
    InvalidInputLength,
    #[error("Invalid Instruction Parsing")]
    InvalidInstructionParse,
    #[error("Invalid Callback Account")]
    InvalidCallbackAccount,
    #[error("Invalid system program")]
    InvalidSystemProgram,
    #[error("Cannot borrow data from account")]
    CannotBorrowData,
    #[error("Invalid Conversion")]
    InvalidConversion,
    #[error("Invalid Callback Program")]
    InvalidCallbackProgram,
    #[error("Invalid Proof")]
    InvalidProof,
    #[error("Proof Verification Failed")]
    ProofVerificationFailed,
    #[error("Invalid Public Inputs")]
    InvalidPublicInputs,
    #[error("Max block height required")]
    MaxBlockHeightRequired,
    #[error("Verify input digest requires digest")]
    InputDigestRequired,
    #[error("Invalid Payer Account")]
    InvalidPayerAccount,
    #[error("Invalid Deployer Account")]
    InvalidDeployerAccount,
    #[error("Invalid Deployment Account")]
    InvalidDeploymentAccount,
    #[error("Invalid Claimer Account")]
    InvalidClaimerAccount,
    #[error("Invalid Claim Account")]
    InvalidClaimAccount,
    #[error("Active claim already exists")]
    ActiveClaimExists,
    #[error("Invalid Stake Account")]
    InvalidStakeAccount,
    #[error("Insufficient Stake")]
    InsufficientStake,
}

impl From<ChannelError> for ProgramError {
    fn from(e: ChannelError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
