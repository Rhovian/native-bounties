use {
    native_mutual_assurance::{
        instruction::CreateBountyArgs, instruction::PoidhInstruction,
        processor::process_instruction, utils::pda::find_bounty_account, ID,
    },
    solana_program::{
        hash::Hash,
        instruction::{AccountMeta, Instruction},
        system_program,
    },
    solana_program_test::{processor, BanksClient, ProgramTest},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
};

pub async fn setup() -> (BanksClient, Keypair, Keypair, Pubkey, Hash) {
    let program_test = ProgramTest::new("Poidh", ID, processor!(process_instruction));
    let mint = Keypair::new();
    let (banks_client, payer, recent_blockhash) = program_test.start().await;

    let (bounty_account, _) = find_bounty_account(&payer.pubkey(), &mint.pubkey());

    (banks_client, payer, mint, bounty_account, recent_blockhash)
}

pub fn create_bounty_instruction(
    payer: &Keypair,
    bounty_account: Pubkey,
    mint: Pubkey,
    name: String,
    description: String,
    amount: u64,
) -> [Instruction; 1] {
    [Instruction::new_with_borsh(
        ID,
        &PoidhInstruction::CreateBounty(CreateBountyArgs {
            name,
            description,
            amount,
        }),
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(bounty_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )]
}

pub fn close_bounty_instruction(
    payer: &Keypair,
    bounty_account: Pubkey,
    mint: Pubkey,
) -> [Instruction; 1] {
    [Instruction::new_with_borsh(
        ID,
        &PoidhInstruction::CloseBounty,
        vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(bounty_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )]
}
