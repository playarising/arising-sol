use anchor_lang::prelude::*;
use anchor_spl::token::{ TokenAccount };

#[inline(always)]
pub fn is_mint_owner(
    mint: Pubkey,
    check_owner: Pubkey,
    owner_token_account: &Account<TokenAccount>
) -> bool {
    if mint != owner_token_account.mint {
        msg!("is_mint_owner: token mint doesn't match");
        return false;
    }

    if check_owner != owner_token_account.owner {
        msg!("is_mint_owner: owner_token_account owner doesn't match");
        return false;
    }

    if owner_token_account.amount < 1 {
        msg!("is_mint_owner: not enough amount of tokens to be owner");
        return false;
    }

    return true;
}