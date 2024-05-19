use {
    crate::utils::pda::find_bounty_account,
    solana_program::{
        account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, system_program,
    },
};

pub fn create_bounty_checks(
    program_id: &Pubkey,
    funding_account: &AccountInfo,
    bounty_account: &AccountInfo,
    mint: &AccountInfo,
    system_program: &AccountInfo,
    amount: u64,
) -> Result<u8, ProgramError> {
    if system_program.key != &system_program::id() {
        return Err(ProgramError::InvalidArgument);
    }

    if program_id != &crate::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    if amount == 0 {
        return Err(ProgramError::InvalidArgument);
    }

    if !funding_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (bounty_pda, bump) = find_bounty_account(funding_account.key, mint.key);

    if bounty_pda != *bounty_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    if bounty_account.lamports() > 0 || !bounty_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    Ok(bump)
}
