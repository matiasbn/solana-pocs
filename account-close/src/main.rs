use std::mem;

use arrayref::array_refs;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::log::sol_log_compute_units;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::{create_account, create_account_with_seed, transfer};
use solana_program::sysvar::Sysvar;
use solana_program::{msg, system_program};
use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::signers::Signers;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::initialize_mint;

fn transfer_to_owner(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let owner_account = next_account_info(accounts_iter)?;
    let account_id = next_account_info(accounts_iter)?;
    // empty the account_id lamports balance, transfer to owner_account to have a balanced instruction
    **owner_account.try_borrow_mut_lamports()? += account_id.lamports();
    **account_id.try_borrow_mut_lamports()? -= account_id.lamports();
    Ok(())
}

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    match data[0] {
        0 => transfer_to_owner(accounts),
        _ => Err(ProgramError::InvalidArgument),
    }
}

#[tokio::test]
async fn test_close_account() -> ProgramResult {
    let program_id = Pubkey::new_unique();
    let account_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new("poc", program_id, processor!(process_instruction));
    // Step 1: Create an account with some `lamports` balance, and a non-zero `data` field.
    program_test.add_account(
        account_id,
        Account {
            lamports: 1000,
            data: vec![1],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );
    let (mut banks_client, owner_account, mut recent_blockhash) = program_test.start().await;

    let account_id_before_transfer = banks_client.get_account(account_id).await.unwrap().unwrap();

    // Step 2: Check the account state.
    println!(
        "account id before transfer: {:?}",
        account_id_before_transfer
    );

    // Step 3: Withdraw the lamports balance from the account, without modifying the `data` field.
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0_u8],
            vec![
                AccountMeta::new(owner_account.pubkey(), true),
                AccountMeta::new(account_id, false),
                AccountMeta::new(program_id, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
        )],
        Some(&owner_account.pubkey()),
    );
    transaction.sign(&[&owner_account], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Step 4. Check that account is closed.
    let account_id_after_transfer = banks_client.get_account(account_id).await.unwrap();
    println!("account id after transfer: {:?}", account_id_after_transfer);
    Ok(())
}

fn main() {
    println!("hello this is a dummy main")
}
