use {
    crate::{error::PoidhError, state::Bounty, utils::pda::find_bounty_account},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
        program_pack::Pack, pubkey::Pubkey, system_program,
    },
};

pub fn program_checks(program_id: &Pubkey, system_program: &AccountInfo) -> ProgramResult {
    if system_program.key != &system_program::id() {
        return Err(ProgramError::InvalidArgument);
    }

    if program_id != &crate::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

pub fn create_bounty_checks(
    program_id: &Pubkey,
    funding_account: &AccountInfo,
    bounty_account: &AccountInfo,
    mint: &AccountInfo,
    system_program: &AccountInfo,
    amount: u64,
) -> Result<u8, ProgramError> {
    program_checks(program_id, system_program)?;

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

pub fn close_bounty_checks(
    program_id: &Pubkey,
    owner: &AccountInfo,
    bounty_account: &AccountInfo,
    mint: &AccountInfo,
    system_program: &AccountInfo,
) -> ProgramResult {
    program_checks(program_id, system_program)?;

    let bounty_data = Bounty::unpack(&bounty_account.data.borrow())?;
    if !bounty_data.is_initialized {
        return Err(PoidhError::BountyNotInitialized.into());
    }

    if bounty_data.mint != *mint.key {
        return Err(ProgramError::InvalidArgument);
    }

    if bounty_data.owner != *owner.key {
        return Err(PoidhError::InvalidOwner.into());
    }

    if bounty_data.claimer != Pubkey::default() {
        return Err(PoidhError::BountyAlreadyClaimed.into());
    }

    Ok(())
}
