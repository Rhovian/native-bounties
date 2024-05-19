#[cfg(test)]
mod tests {
    use native_mutual_assurance::utils::pda::find_bounty_account;
    use native_mutual_assurance::ID;
    use native_mutual_assurance::{
        instruction::{CreateBountyArgs, PoidhInstruction},
        processor::process_instruction,
    };
    use solana_program::instruction::{AccountMeta, Instruction};
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_close_bounty() {
        let program_test = ProgramTest::new("Poidh", ID, processor!(process_instruction));
        let mint = Keypair::new();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let (bounty_account, _) = find_bounty_account(&payer.pubkey(), &mint.pubkey());

        // Create the bounty
        let mut create_transaction = Transaction::new_with_payer(
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
        create_transaction.sign(&[&payer], recent_blockhash);
        banks_client
            .process_transaction(create_transaction)
            .await
            .unwrap();

        // Close the bounty
        let mut close_transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                ID,
                &PoidhInstruction::CloseBounty,
                vec![
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new(bounty_account, false),
                    AccountMeta::new_readonly(mint.pubkey(), false),
                    AccountMeta::new_readonly(solana_program::system_program::id(), false),
                ],
            )],
            Some(&payer.pubkey()),
        );
        close_transaction.sign(&[&payer], recent_blockhash);
        banks_client
            .process_transaction(close_transaction)
            .await
            .unwrap();

        // Check that the bounty account is closed
        assert!(banks_client
            .get_account(bounty_account)
            .await
            .unwrap()
            .is_none());
    }
}
