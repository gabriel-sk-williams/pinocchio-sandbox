
use std::io::Cursor;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::{Contract, ContractAccount}; // PayoutStatus, ApprovalState, VersusContract, Wager};

use pinocchio::{
    account_info::{AccountInfo},
    program_error::ProgramError,
    pubkey::Pubkey,
    instruction::{Seed, Signer},
    sysvars::{rent::Rent, Sysvar},
    syscalls::{sol_sha256},
    pubkey::find_program_address,
    ProgramResult,
    msg,
};

use sha2::{Sha256, Digest};

use pinocchio_system::instructions::{CreateAccount}; //, Transfer as SystemTransfer

pub fn create_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    contract: Contract,
) -> ProgramResult {
    let [payer, new_account, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    msg!("so far so good!");

    // Verify account ownership and signing
    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Create wager account
    let rent = Rent::get()?;
    let creator_allocation = 32;
    let deadline_allocation = 8;
    let title_allocation = 4 + contract.title.len();
    let terms_allocation = 4 + contract.terms.len();
    let contract_allocation = creator_allocation + deadline_allocation + title_allocation + terms_allocation;
    let required_lamports = rent.minimum_balance(contract_allocation);

    let form = format!("alloc {} {}", contract_allocation, required_lamports);
    msg!(&form);

    // Derive PDA
    /*
    let mut hash_result = [0u8; 32];
    let bytes = contract.terms.as_bytes();

    unsafe {
        sol_sha256(
            bytes.as_ptr(), 
            bytes.len() as u64, 
            hash_result.as_mut_ptr()
        );
    }
    */

    let terms_bytes = contract.terms.as_bytes();
    let seed_bytes = if terms_bytes.len() >= 32 {
        &terms_bytes[..32]
    } else {
        terms_bytes
    };

    msg!("hashing complete");

    let (pda, bump) = find_program_address(
        &[
            //&hash_result[..],
            payer.key().as_ref(),
            seed_bytes,
        ], 
        program_id
    );
    
    let signer_seeds = [
        //Seed::from(&hash_result[..]),
        Seed::from(payer.key().as_ref()),
        Seed::from(seed_bytes),
        Seed::from(core::slice::from_ref(&bump)), // includes bump
    ];

    let signer = Signer::from(&signer_seeds);

    if pda != *new_account.key() {
        return Err(ProgramError::InvalidArgument);
    }

    msg!("pda clear {:?}", pda);

    CreateAccount {
        from: payer,
        to: new_account,
        lamports: required_lamports,
        space: contract_allocation as u64,
        owner: program_id,
    }
    .invoke_signed(&[signer])?;

    let contract_account = ContractAccount {
        creator: *payer.key(), //.as_ref(),
        contract: contract,
    };

    msg!("account created!");

    let mut account_data = new_account.try_borrow_mut_data()?;
    // account_data[0..32].copy_from_slice(payer.key().as_ref());
    // account_data[32..].copy_from_slice(instruction_data);
    
    let mut cursor = Cursor::new(&mut account_data[..]);
    contract_account.serialize(&mut cursor)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}