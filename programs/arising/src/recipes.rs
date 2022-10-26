use anchor_lang::prelude::*;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;

const CRAFT_RECIPE_PREFIX: &str = "arsing_craft";
const FORGE_RECIPE_PREFIX: &str = "arising_forge_recipe";

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
#[instruction(bump: u8, id: u64)]
pub struct AddForgeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.forge_recipes + 1) == id @ ArisingError::InvalidForgeRecipeID
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

#[derive(Accounts)]
pub struct UpdateCraftRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub craft_recipe: Account<'info, CraftRecipe>,
}

#[derive(Accounts)]
#[instruction(bump: u8, id: u64)]
pub struct AddCraftRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.craft_recipes + 1) == id @ ArisingError::InvalidCraftRecipeID
    )]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [CRAFT_RECIPE_PREFIX.as_bytes(), &id.to_le_bytes()],
        bump,
        space = RECIPE_SIZE
    )]
    pub craft_recipe: Account<'info, CraftRecipe>,

    pub system_program: Program<'info, System>,
}

enum ResourceType {
    RAW = 1,
    BASIC,
    ITEM,
}

/// The size of a craft and forge recipe.
pub const RECIPE_SIZE: usize =
    8 + // discriminator
    64 + // id
    24 + // name
    640 + // materials
    640 + // materials_amounts
    640 + // materials_types
    BASE_STATS_SIZE + // stats_required
    BASE_STATS_SIZE + // stats_sacrificed
    64 + // cooldown
    64 + // level_required
    64 + // item_rewarded
    64 + // item_rewarded_amount
    64 + // item_rewarded_type
    1; // available

/// The full metadata information for a recipe.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Recipe {
    pub id: u64,
    pub name: String,
    pub materials: [u64; 10],
    pub materials_amounts: [u64; 10],
    pub materials_types: [u64; 10],
    pub stats_required: BaseStats,
    pub stats_sacrificed: BaseStats,
    pub cooldown: u64,
    pub level_required: u64,
    pub item_rewarded: u64,
    pub item_rewarded_amount: u64,
    pub item_rewarded_type: u64,
    pub available: bool,
}

/// Forge recipes account storage
#[account]
pub struct ForgeRecipe {
    pub recipe: Recipe,
}

/// Craft recipes account storage
#[account]
pub struct CraftRecipe {
    pub recipe: Recipe,
}