


use borsh::{BorshDeserialize, BorshSerialize};

use pinocchio::{
  pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Contract {
    pub deadline: u64,          // 8 bytes
    pub title: String,          // 4 + length
    pub terms: String,          // 4 + length
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ContractAccount {
    pub creator: Pubkey,        // 32 bytes
    pub contract: Contract,
}

/*
pub struct Wager {
    pub format: Format,
    pub stake: u64,
    pub lock_by: u64,
    pub headline: String,       // 4 + length
    pub terms: String,          // 4 + length
}

pub enum Format {
    Versus,
    Table,
    Continuous
}

pub enum WagerState {
    Active,
    Disputed,
    Resolved,
    Closed
}

pub enum ApprovalState {
    Pending,
    Landed,
    Missed,
    Push,
}   
*/