
use solana_program::{
    pubkey::Pubkey,
};


pub struct Wager {
    pub format: Format,
    pub stake: u64,
    pub lock_by: u64,
    pub resolves_by: u64,
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