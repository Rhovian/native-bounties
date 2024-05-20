#[cfg(test)]
mod utils;
use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::Transaction};

#[tokio::test]
async fn test_close_bounty() {
    let (mut banks_client, payer, mint, bounty_account, recent_blockhash) = utils::setup().await;

    let mut create_transaction = Transaction::new_with_payer(
        &utils::create_bounty_instruction(
            &payer,
            bounty_account,
            mint.pubkey(),
            "Test Bounty".to_string(),
            "A test bounty".to_string(),
            100_000_000,
        ),
        Some(&payer.pubkey()),
    );
    create_transaction.sign(&[&payer], recent_blockhash);
    banks_client
        .process_transaction(create_transaction)
        .await
        .unwrap();

    let mut close_transaction = Transaction::new_with_payer(
        &utils::close_bounty_instruction(&payer, bounty_account, mint.pubkey()),
        Some(&payer.pubkey()),
    );
    close_transaction.sign(&[&payer], recent_blockhash);
    banks_client
        .process_transaction(close_transaction)
        .await
        .unwrap();

    assert!(banks_client
        .get_account(bounty_account)
        .await
        .unwrap()
        .is_none());
}
