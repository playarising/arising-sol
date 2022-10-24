use anchor_lang::prelude::*;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;

const CRAFT_RECIPE_PREFIX: &str = "arsing_craft";
const UPGRADE_RECIPE_PREFIX: &str = "arsing_upgrade";
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
#[instruction(bump: u8, id: u16)]
pub struct AddForgeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [FORGE_RECIPE_PREFIX.as_bytes(), &id.to_be_bytes()],
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
#[instruction(bump: u8, id: u16)]
pub struct AddCraftRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [CRAFT_RECIPE_PREFIX.as_bytes(), &id.to_be_bytes()],
        bump,
        space = RECIPE_SIZE
    )]
    pub craft_recipe: Account<'info, CraftRecipe>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUpgradeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub upgrade_recipe: Account<'info, UpgradeRecipe>,
}

#[derive(Accounts)]
#[instruction(bump: u8, id: u16)]
pub struct AddUpgradeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [UPGRADE_RECIPE_PREFIX.as_bytes(), &id.to_be_bytes()],
        bump,
        space = RECIPE_SIZE
    )]
    pub upgrade_recipe: Account<'info, UpgradeRecipe>,

    pub system_program: Program<'info, System>,
}

/// The size of a recipe for craft, upgrade, quests and forge
pub const RECIPE_SIZE: usize =
    8 + // discriminator
    16 + // id
    24 + // name
    24 + // description
    80 + // materials
    80 + // materials_amounts
    BASE_STATS_SIZE + // stats_required
    32 + // cooldown
    16 + // level_required
    16 + // item_rewarded
    16 + // item_rewarded_amount
    1; // available

/// The full metadata information for a recipe.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Recipe {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub materials: [u16; 5],
    pub materials_amounts: [u16; 5],
    pub stats_required: BaseStats,
    pub stats_sacrificed: BaseStats,
    pub cooldown: u32,
    pub level_required: u16,
    pub item_rewarded: u16,
    pub item_rewarded_amount: u16,
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

/// Upgrade recipes account storage
#[account]
pub struct UpgradeRecipe {
    pub recipe: Recipe,
}