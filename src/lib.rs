// lib.rs
// #![no_std]

use pinocchio::{
  account_info::AccountInfo,
  // lazy_entrypoint,
  entrypoint,
  msg,
  ProgramResult,
  pubkey::Pubkey,
};

pub mod state;
pub mod instruction;
pub mod processor;

use processor::{
    //get_wager, 
    create_contract, 
    //process_deposit,
    //update_belief,
    //lock_status,
    //set_approval,
    //render_payouts,
};

use instruction::WagerInstruction;


entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Unpack instruction data
    let instruction = WagerInstruction::unpack(instruction_data)?;

    match instruction {
		WagerInstruction::CreateContract { contract } => {
            msg!("creating contract...");
            create_contract(program_id, accounts, contract)
        }
    }
}

/*
pub fn get_wager(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
	msg!("Getting wager lol");
    // msg!("id {:?}", program_id);

    //let accounts_iter = &mut accounts.iter();
    //let wager_account = next_account_info(accounts_iter)?;

    // Print SOL balance (in lamports)
    //msg!("wager_account lamports: {}", wager_account.lamports());

    // Deserialize wager
    //let data = &wager_account.data.borrow_mut();
    //let wager_data = Wager::try_from_slice(&data);
    //msg!("result {:?}", wager_data);

    Ok(())
}
*/