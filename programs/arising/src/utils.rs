use anchor_lang::{ prelude::*, solana_program::clock };
use std::convert::TryInto;

pub fn now() -> Result<u64> {
    Ok(clock::Clock::get()?.unix_timestamp.try_into().unwrap())
}