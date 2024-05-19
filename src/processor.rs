use {
    crate::{
        instruction::{CreateBountyArgs, PoidhInstruction},
        state::Bounty,
        utils::{
            create_account,
            transfer,
            pda::BOUNTY,
        },
        validation::create_bounty_checks,
    },
    borsh::BorshDeserialize,
    solana_program::{
        account_info::next_account_info, account_info::AccountInfo, clock::Clock,
        entrypoint::ProgramResult, msg, program_pack::Pack,
        pubkey::Pubkey, sysvar::Sysvar,
    },
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PoidhInstruction::try_from_slice(instruction_data)?;
    match instruction {
        PoidhInstruction::CreateBounty(args) => {
            msg!("Instruction: CreateBounty");
            process_create_bounty(program_id, accounts, args)
        }
    }
}

pub fn process_create_bounty(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateBountyArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let funding_account = next_account_info(account_info_iter)?;
    let bounty_account = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let bump = create_bounty_checks(
        program_id,
        funding_account,
        bounty_account,
        mint,
        system_program,
        args.amount,
    )?;

    // construct seeds with bump
    let seeds = &[
        BOUNTY.as_bytes(),
        &funding_account.key.as_ref(),
        &mint.key.as_ref(),
        &[bump],
    ];

    create_account(
        program_id,
        bounty_account,
        system_program,
        funding_account,
        Bounty::LEN,
        seeds,
    )?;

    transfer(funding_account, bounty_account, system_program, args.amount)?;

    let bounty_data = Bounty {
        owner: *funding_account.key,
        mint: *mint.key,
        name: args.name,
        description: args.description,
        amount: args.amount,
        claimer: Pubkey::default(),
        created_at: Clock::get()?.unix_timestamp as u64,
        claim_id: 0,
        is_initialized: true,
    };

    bounty_data.pack_into_slice(&mut bounty_account.data.borrow_mut());

    Ok(())
}
