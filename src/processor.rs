



pub fn get_wager(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    msg!("id {:?}", program_id);

    let accounts_iter = &mut accounts.iter();
    let wager_account = next_account_info(accounts_iter)?;

    // Print SOL balance (in lamports)
    msg!("wager_account lamports: {}", wager_account.lamports());

    // Deserialize wager
    let data = &wager_account.data.borrow_mut();
    let wager_data = Wager::try_from_slice(&data);
    msg!("result {:?}", wager_data);

    Ok(())
}