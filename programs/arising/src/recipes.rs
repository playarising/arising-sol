use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;
use crate::characters::*;
use crate::utils::*;

const CRAFT_RECIPE_PREFIX: &str = "arsing_craft";
const FORGE_RECIPE_PREFIX: &str = "arising_forge_recipe";

#[inline(always)]
// Check if the forge recipe is available globaly.
pub fn is_forge_recipe_available(recipe: &Account<ForgeRecipe>) -> bool {
    return recipe.recipe.available;
}

#[inline(always)]
pub fn is_forge_available_for_character(character: &Account<Character>) -> bool {
    // Check if this is the first use of the character forge.
    if character.forge.cooldown == 0 {
        return true;
    }

    // Check if the cooldown has passed and the forge has been claimed.
    return character.forge.cooldown <= now() && character.forge.last_recipe_claimed;
}

#[inline(always)]
pub fn is_forge_claimable_for_character(character: &Account<Character>) -> bool {
    return (
        character.forge.cooldown <= now() &&
        !character.forge.last_recipe_claimed &&
        character.forge.last_recipe != 0
    );
}

#[inline(always)]
// Check if the recipe is available globally
pub fn is_craft_recipe_available(recipe: &Account<CraftRecipe>) -> bool {
    return recipe.recipe.available;
}

#[inline(always)]
pub fn is_craft_available_for_character(character: &Account<Character>) -> bool {
    // Check if this is the first use of the character craft.
    if character.craft.cooldown == 0 {
        return true;
    }

    // Check if the cooldown has passed and the craft has been claimed.
    return character.craft.cooldown <= now() && character.craft.last_recipe_claimed;
}

#[inline(always)]
pub fn is_craft_claimable_for_character(character: &Account<Character>) -> bool {
    return (
        character.craft.cooldown <= now() &&
        !character.craft.last_recipe_claimed &&
        character.craft.last_recipe != 0
    );
}

#[inline(always)]
pub fn consume_materials(
    character: &mut Account<Character>,
    materials: &[u64; 10],
    amounts: &[u64; 10],
    types: &[u64; 10]
) {
    let mut i: usize = 0;
    while i < 10 {
        let material = materials[i];
        let amount = amounts[i];
        let material_type = types[i];

        if material == 0 {
            continue;
        }

        if material_type == (ResourceType::Basic as u64) {
            consume_basic_material(character, material, amount);
        }

        if material_type == (ResourceType::Raw as u64) {
            consume_raw_material(character, material, amount);
        }

        i += 1;
    }
}

#[inline(always)]
pub fn has_enough_materials(
    character: &Account<Character>,
    materials: &[u64; 10],
    amounts: &[u64; 10],
    types: &[u64; 10]
) -> bool {
    let mut i: usize = 0;
    while i < 10 {
        let material = materials[i];
        let amount_required = amounts[i];
        let material_type = types[i];

        let amount = get_material_amount(character, material, material_type);

        if amount_required > amount {
            return false;
        }

        i += 1;
    }

    return true;
}

#[inline(always)]
pub fn consume_raw_material(character: &mut Account<Character>, material: u64, amount: u64) {
    if material > 16 {
        return;
    }

    character.raw_materials[(material as usize) - 1] -= amount;

    return;
}

#[inline(always)]
pub fn consume_basic_material(character: &mut Account<Character>, material: u64, amount: u64) {
    if material > 15 {
        return;
    }
    character.basic_materials[(material as usize) - 1] -= amount;

    return;
}

#[inline(always)]
pub fn get_material_amount(
    character: &Account<Character>,
    material: u64,
    material_type: u64
) -> u64 {
    if material_type == (ResourceType::Basic as u64) {
        return get_basic_material_amount(character, material);
    }

    if material_type == (ResourceType::Raw as u64) {
        return get_raw_material_amount(character, material);
    }

    return 0;
}

#[inline(always)]
pub fn get_raw_material_amount(character: &Account<Character>, material: u64) -> u64 {
    if material > 16 {
        return 0;
    }

    return character.raw_materials[(material as usize) - 1];
}

#[inline(always)]
pub fn get_basic_material_amount(character: &Account<Character>, material: u64) -> u64 {
    if material > 15 {
        return 0;
    }

    return character.basic_materials[(material as usize) - 1];
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
#[instruction(bump: u8, id: u64)]
pub struct AddForgeRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.forge_recipes + 1) == id @ ForgeError::InvalidID
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
pub struct CraftAccess<'info> {
    #[account(mut,
        constraint = is_mint_owner(character.mint, payer.key(), &character_token_account) @ ArisingError::InvalidOwner)]
    payer: Signer<'info>,

    #[account(mut)]
    pub character: Account<'info, Character>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub craft_recipe: Account<'info, CraftRecipe>,
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
        constraint = (config.craft_recipes + 1) == id @ CraftError::InvalidID
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
    Raw = 1,
    Basic,
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