use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::characters::*;
use crate::codex::*;
use crate::config::*;
use crate::errors::*;

const FORGE_RECIPE_PREFIX: &str = "arising_forge_recipe";

#[inline(always)]
pub fn forge_reward(
    character: &mut Account<Character>,
    material: u32,
    amount: u32,
    material_type: u16,
) {
    if material == 0 {
        return;
    }

    if material_type == (ResourceType::Basic as u16) {
        character.basic[(material as usize) - 1] += amount;
    }

    if material_type == (ResourceType::Raw as u16) {
        character.raw[(material as usize) - 1] += amount;
    }

    return;
}

#[derive(Accounts)]
pub struct ForgeAccess<'info> {
    #[account(mut,
        constraint = is_mint_owner(character.mint, payer.key(), &character_token_account) @ ArisingError::InvalidOwner)]
    payer: Signer<'info>,

    #[account(mut)]
    pub character: Account<'info, Character>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub forge_recipe: Account<'info, ForgeRecipe>,
}

#[derive(Accounts)]
pub struct UpdateForgeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub forge_recipe: Account<'info, ForgeRecipe>,
}

#[derive(Accounts)]
#[instruction(bump: u8, id: u32)]
pub struct AddForgeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.forge_recipes + 1) == id.into() @ ForgeError::InvalidID
    )]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [FORGE_RECIPE_PREFIX.as_bytes(), &id.to_le_bytes()],
        bump,
        space = RECIPE_SIZE
    )]
    pub forge_recipe: Account<'info, ForgeRecipe>,

    pub system_program: Program<'info, System>,
}

/// Forge recipes account storage
#[account]
pub struct ForgeRecipe {
    pub recipe: Recipe,
}
