#[cfg(test)]
pub mod utils;

use native_mutual_assurance::ID;
use native_mutual_assurance::{
    instruction::{CreateBountyArgs, PoidhInstruction},
    state::Bounty,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
};
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::Transaction};

#[tokio::test]
async fn test_create_bounty() {
    let (mut banks_client, payer, mint, bounty_account, recent_blockhash) = utils::setup().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            ID,
            &PoidhInstruction::CreateBounty(CreateBountyArgs {
                name: "Test Bounty".to_string(),
                description: "A test bounty".to_string(),
                amount: 100_000_000,
            }),
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(bounty_account, false),
                AccountMeta::new_readonly(mint.pubkey(), false),
                AccountMeta::new_readonly(solana_program::system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    let bounty_account_info = banks_client
        .get_account(bounty_account)
        .await
        .expect("get_account")
        .expect("bounty_account not found");

    assert_eq!(bounty_account_info.owner, ID);
    assert_eq!(bounty_account_info.data.len(), Bounty::LEN);
    assert_eq!(
        bounty_account_info.lamports,
        Rent::default().minimum_balance(Bounty::LEN) + 100_000_000
    );

    let bounty = Bounty::unpack_from_slice(&bounty_account_info.data).unwrap();
    assert!(bounty.is_initialized);
    assert_eq!(bounty.owner, payer.pubkey());
    assert_eq!(bounty.mint, mint.pubkey());
    assert_eq!(bounty.name, "Test Bounty");
    assert_eq!(bounty.description, "A test bounty");
    assert_eq!(bounty.amount, 100_000_000);
    assert_eq!(bounty.claimer, Pubkey::default());
    assert_eq!(bounty.claim_id, 0);
}
