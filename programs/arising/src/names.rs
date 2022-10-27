use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;
use crate::checks::*;
use crate::utils::*;

const CHARACTER_NAME_PREFIX: &str = "arising_character_name";

#[derive(Accounts)]
#[instruction(mint: Pubkey, bump: u8)]
pub struct CreateName<'info> {
    #[account(mut,
        constraint = is_mint_owner(character.mint, payer.key(), &character_token_account) @ ArisingError::InvalidCharacterOwner)]
    payer: Signer<'info>,

    #[account(mut)]
    pub character: Account<'info, Character>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        seeds = [CHARACTER_NAME_PREFIX.as_bytes(), &mint.to_bytes()],
        bump,
        space = CHARACTER_NAME_ACCOUNT_SIZE
    )]
    pub name: Account<'info, CharacterName>,

    pub system_program: Program<'info, System>,
}

/// The size of the character name account in bytes.
pub const CHARACTER_NAME_ACCOUNT_SIZE: usize =
    8 + // discriminator
    24; // name

/// The full metadata information for an Arising character name.
#[account]
pub struct CharacterName {
    pub name: String,
}