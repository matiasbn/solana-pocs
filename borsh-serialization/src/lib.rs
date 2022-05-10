use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_memory::sol_memcpy;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct PocStruct {
    data1: u8,
    data2: String,
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    sol_memcpy(*account.try_borrow_mut_data().unwrap(), &data, data.len());
    Ok(())
}

#[tokio::test]
async fn test_your_poc() {
    // Create a pubkey
    let program_id = Pubkey::new_unique();
    let account_id = Pubkey::new_unique();

    let mut program_test = ProgramTest::new("poc", program_id, processor!(process_instruction));

    // Set the account data to send
    let account_data = PocStruct {
        data1: 20,
        data2: "gm frens".to_string(),
    };

    let account_data_serialized = account_data.try_to_vec().unwrap();

    // Create the account, data empty, length equal to data serialized
    program_test.add_account(
        account_id,
        Account {
            lamports: 10000,
            data: vec![0_u8; account_data_serialized.len()],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    let (mut banks_client, owner_account, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            // &[0_u8],
            &account_data,
            vec![AccountMeta::new(account_id, false)],
        )],
        Some(&owner_account.pubkey()),
    );

    transaction.sign(&[&owner_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    let account = banks_client.get_account(account_id).await.unwrap().unwrap();
    println!("account: {:?}", account);
    let deserialized_account_data: PocStruct = PocStruct::try_from_slice(&account.data).unwrap();
    println!("deserialized_data: {:?}", deserialized_account_data);
    assert_eq!(deserialized_account_data.data1, account_data.data1);
    assert_eq!(deserialized_account_data.data2, account_data.data2);
}
