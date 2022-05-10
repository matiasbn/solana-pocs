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

// 1. Create a struct with a couple fields in it to store different data.
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
async fn test_borsh_serialization() {
    let program_id = Pubkey::new_unique();
    let account_id = Pubkey::new_unique();

    let mut program_test = ProgramTest::new("poc", program_id, processor!(process_instruction));

    // 2. Cast an instance of the struct with some values.
    let account_data = PocStruct {
        data1: 20,
        data2: "gm frens".to_string(),
    };

    let account_data_serialized = account_data.try_to_vec().unwrap();
    // 3. Create an account to store this struct.
    program_test.add_account(
        account_id,
        Account {
            lamports: 10000,
            // length equal to serialized data length
            data: vec![0_u8; account_data_serialized.len()],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );

    let (mut banks_client, owner_account, recent_blockhash) = program_test.start().await;

    // 4. Modify the account content by sending a transaction, using the serialized instance as data.
    let mut transaction = Transaction::new_with_payer(
        // new_with_borsh will serialize the data before sending
        // by calling .try_to_vec().unwrap() first
        &[Instruction::new_with_borsh(
            program_id,
            &account_data,
            vec![AccountMeta::new(account_id, false)],
        )],
        Some(&owner_account.pubkey()),
    );

    transaction.sign(&[&owner_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    let account = banks_client.get_account(account_id).await.unwrap().unwrap();
    println!("account: {:?}", account);
    // 5. Read the data content and deserialize it with Borsh.
    let deserialized_account_data: PocStruct = PocStruct::try_from_slice(&account.data).unwrap();
    println!("deserialized_data: {:?}", deserialized_account_data);
    // 6. Compare the deserialized data with the casted instance.
    assert_eq!(deserialized_account_data.data1, account_data.data1);
    assert_eq!(deserialized_account_data.data2, account_data.data2);
}
