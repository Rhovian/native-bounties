#[cfg(test)]
mod tests {
    use super::*;
    use native_mutual_assurance::{
        instruction::{CreateBountyArgs, PoidhInstruction},
        processor::process_instruction,
        state::Bounty,
    };
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction, sysvar,
    };
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_create_bounty() {
        let program_id = Pubkey::new_unique();
        let mut program_test =
            ProgramTest::new("poidh", program_id, processor!(process_instruction));

        let funding_account = Keypair::new();
        let bounty_account = Keypair::new();
        let mint = Keypair::new();
        let payment_mint = Keypair::new();
        let funding_account_token = Keypair::new();
        let bounty_account_token = Keypair::new();

        program_test.add_account(
            funding_account.pubkey(),
            Account {
                lamports: Rent::default().minimum_balance(0),
                ..Default::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let rent = banks_client.get_rent().await.unwrap();
        let bounty_account_rent = rent.minimum_balance(Bounty::LEN);

        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &bounty_account.pubkey(),
                    bounty_account_rent,
                    Bounty::LEN as u64,
                    &program_id,
                ),
                Instruction::new_with_borsh(
                    program_id,
                    &PoidhInstruction::CreateBounty(CreateBountyArgs {
                        name: "Test Bounty".to_string(),
                        description: "A test bounty".to_string(),
                        amount: 100,
                    }),
                    vec![
                        AccountMeta::new(funding_account.pubkey(), true),
                        AccountMeta::new(bounty_account.pubkey(), false),
                        AccountMeta::new_readonly(mint.pubkey(), false),
                        AccountMeta::new_readonly(payment_mint.pubkey(), false),
                        AccountMeta::new(funding_account_token.pubkey(), false),
                        AccountMeta::new(bounty_account_token.pubkey(), false),
                        AccountMeta::new_readonly(spl_token::id(), false),
                        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
                        AccountMeta::new_readonly(solana_program::system_program::id(), false),
                    ],
                ),
            ],
            Some(&payer.pubkey()),
        );

        transaction.sign(
            &[&payer, &funding_account, &bounty_account],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        let bounty_account_info = banks_client
            .get_account(bounty_account.pubkey())
            .await
            .expect("get_account")
            .expect("bounty_account not found");

        assert_eq!(bounty_account_info.owner, program_id);
        assert_eq!(bounty_account_info.data.len(), Bounty::LEN);

        let bounty = Bounty::unpack_from_slice(&bounty_account_info.data).unwrap();
        assert!(bounty.is_initialized);
        assert_eq!(bounty.owner, funding_account.pubkey());
        assert_eq!(bounty.mint, mint.pubkey());
        assert_eq!(bounty.payment_mint, payment_mint.pubkey());
        assert_eq!(bounty.name, "Test Bounty");
        assert_eq!(bounty.description, "A test bounty");
        assert_eq!(bounty.amount, 100);
        assert_eq!(bounty.claimer, Pubkey::default());
        assert_eq!(bounty.claim_id, 0);
    }
}
