use anchor_lang::prelude::*;

#[inline(always)]
pub fn is_mint_owner(payer: Pubkey, mint: Pubkey) -> Result<bool> {
    let mut owner = false;
    Ok(owner)
}