use anchor_lang::{ prelude::*, solana_program::clock };

pub fn now() -> u64 {
    return clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
}