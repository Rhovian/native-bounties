pub mod pda;

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn create_account<'a>(
    program_id: &Pubkey,
    new_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    size: usize,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    let lamports_required = (Rent::get()?).minimum_balance(size);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            lamports_required,
            size as u64,
            program_id,
        ),
        &[payer.clone(), new_account.clone(), system_program.clone()],
        &[&signer_seeds],
    )?;

    Ok(())
}

pub fn transfer<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        &system_instruction::transfer(from.key, to.key, amount),
        &[from.clone(), to.clone(), system_program.clone()],
    )?;

    Ok(())
}
