use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;
use crate::characters::*;

const CRAFT_RECIPE_PREFIX: &str = "arsing_craft";
const FORGE_RECIPE_PREFIX: &str = "arising_forge_recipe";

#[inline(always)]
pub fn forge_reward(
    character: &mut Account<Character>,
    material: u32,
    amount: u32,
    material_type: u16
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

#[inline(always)]
pub fn consume_materials(
    character: &mut Account<Character>,
    materials: &[u32; 10],
    amounts: &[u32; 10],
    types: &[u16; 10]
) {
    let mut i: usize = 0;

    loop {
        if i >= 10 {
            break;
        }

        let material = materials[i];
        let amount = amounts[i];
        let material_type = types[i];

        if material != 0 {
            if material_type == (ResourceType::Basic as u16) {
                character.basic[(material - 1) as usize] -= amount;
            }

            if material_type == (ResourceType::Raw as u16) {
                character.raw[(material - 1) as usize] -= amount;
            }
        }

        i += 1;
    }
}

#[inline(always)]
pub fn has_enough_materials(
    character: &Account<Character>,
    materials: &[u32; 10],
    amounts: &[u32; 10],
    types: &[u16; 10]
) -> bool {
    let mut i: usize = 0;
    loop {
        if i >= 10 {
            break;
        }

        let material = materials[i];
        let amount_required = amounts[i];
        let material_type = types[i];

        if material != 0 {
            let mut material_amount: u32 = 0;

            if material_type == (ResourceType::Basic as u16) {
                material_amount = character.basic[(material as usize) - 1];
            }

            if material_type == (ResourceType::Raw as u16) {
                material_amount = character.raw[(material as usize) - 1];
            }

            if amount_required > material_amount {
                return false;
            }
        }

        i += 1;
    }

    return true;
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
#[instruction(bump: u8, id: u32)]
pub struct AddCraftRecipe<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.craft_recipes + 1) == id.into() @ CraftError::InvalidID
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
    32 + // id
    24 + // name
    320 + // materials
    320 + // materials_amounts
    150 + // materials_types
    BASE_STATS_SIZE + // stats_required
    BASE_STATS_SIZE + // stats_sacrificed
    32 + // cooldown
    16 + // level_required
    32 + // item_rewarded
    32 + // item_rewarded_amount
    16 + // item_rewarded_type
    1; // available

/// The full metadata information for a recipe.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub materials: [u32; 10],
    pub materials_amounts: [u32; 10],
    pub materials_types: [u16; 10],
    pub stats_required: BaseStats,
    pub stats_sacrificed: BaseStats,
    pub cooldown: u32,
    pub level_required: u16,
    pub item_rewarded: u32,
    pub item_rewarded_amount: u32,
    pub item_rewarded_type: u16,
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