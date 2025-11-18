use pinocchio::{
  account_info::AccountInfo,
  // lazy_entrypoint,
  entrypoint,
  msg,
  ProgramResult,
  // program_error::ProgramError,
  pubkey::Pubkey,
  pubkey::find_program_address,
};

//pub mod instruction;
//pub mod state;
//pub mod processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

	let rating: u64 = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
   	let message = format!("You rated {}/10!", rating);
	msg!(&message);

	/*
    // Unpack instruction data
    let instruction = WagerInstruction::unpack(instruction_data)?;
    // msg!("instruct {:?}", instruction);

    match instruction {
		WagerInstruction::GetWager => {
			get_wager(program_id, accounts)
		}
    }
	*/

    Ok(())
}

/*
pub enum WagerInstruction {
	GetWager,
	//CreateWager { contract: Wager}
}

impl WagerInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Get instruction variant from first byte
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
 
        // Match instruction type and parse remaining bytes based on variant
        match variant {
            0 => {
                Ok(Self::GetWager)
            }
            _ => {
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}

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