use {
    native_mutual_assurance::{
        processor::process_instruction, utils::pda::find_bounty_account, ID,
    },
    solana_program::hash::Hash,
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
