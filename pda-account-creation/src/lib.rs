use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction::create_account;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "Transaction::sign failed with error NotEnoughSigner")]
async fn test_create_pda() {
    // Step 1. Derive a PDA address from a `account_1`, without creating the PDA account.
    let account_1 = Pubkey::new_unique();
    let (account_1_pda, _nonce) = Pubkey::find_program_address(&["poc_pad".as_ref()], &account_1);
    let program_test = ProgramTest::new("poc", account_1, processor!(process_instruction));
    let (mut banks_client, signer, recent_blockhash) = program_test.start().await;

    // Step 2. Invoke a `create_account` instruction to create the PDA account, but with an `owner` different from `account_1`.
    let data_length = 1;
    let rent_exemption_amount = banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(data_length);

    let mut create_account_transaction = Transaction::new_with_payer(
        &[create_account(
            &signer.pubkey(),
            &account_1_pda,
            rent_exemption_amount,
            1,
            &signer.pubkey(),
        )],
        Some(&signer.pubkey()),
    );
    create_account_transaction.sign(&[&signer], recent_blockhash);

    // Transaction should fail because to_pubkey is not signer
    banks_client
        .process_transaction(create_account_transaction)
        .await
        .unwrap();
}

// The next test is intended to show that, by passing a to_pubkey with a signature from the corresponding private key, we can create an account
#[tokio::test]
async fn test_create_account_with_keypair() {
    let account_1 = Pubkey::new_unique();
    let keypair_1 = Keypair::new();
    let program_test = ProgramTest::new("poc", account_1, processor!(process_instruction));
    let (mut banks_client, signer, recent_blockhash) = program_test.start().await;

    let data_length = 1;
    let rent_exemption_amount = banks_client
        .get_rent()
        .await
        .unwrap()
        .minimum_balance(data_length);

    let mut create_account_transaction = Transaction::new_with_payer(
        &[create_account(
            &signer.pubkey(),
            // we know the private key of keypair_1.pubkey()
            &keypair_1.pubkey(),
            rent_exemption_amount,
            1,
            &signer.pubkey(),
        )],
        Some(&signer.pubkey()),
    );

    // we should add the keypair_1 as keypair, so the transaction is signed and accepted
    create_account_transaction.sign(&[&signer, &keypair_1], recent_blockhash);

    // Transaction should succeed
    banks_client
        .process_transaction(create_account_transaction)
        .await
        .unwrap();

    let created_account = banks_client
        .get_account(keypair_1.pubkey())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(created_account.lamports, rent_exemption_amount);
    assert_eq!(created_account.owner, signer.pubkey());
    assert_eq!(created_account.data.len(), data_length);
}
