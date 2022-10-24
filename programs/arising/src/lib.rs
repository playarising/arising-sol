mod config;
mod utils;
mod characters;
mod errors;
mod codex;
mod recipes;

use anchor_lang::prelude::*;

use config::*;
use characters::*;
use recipes::*;
use errors::*;
use codex::*;

declare_id!("GT1koQQwD6ZV6bxciNSwC3YFDHiByySKZbQ2MQJF4GWp");

#[program]
pub mod arising {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _bump: u8) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.initialized = true;
        config.paused = true;
        config.authority = ctx.accounts.authority.unsigned_key().clone();
        config.seconds_between_refreshes = 86_400; // 1 day
        config.seconds_between_paid_refreshes = 86_400; // 1 day
        config.experience_multiplier = 1;
        config.max_characters = 30_000;
        config.forge_recipes = 0;
        config.craft_recipes = 0;
        config.upgrade_recipes = 0;

        Ok(())
    }

    pub fn set_paused(ctx: Context<SetPause>, paused: bool) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.paused = paused;
        msg!("Changing pause status to {}", paused);

        Ok(())
    }

    pub fn add_character(ctx: Context<AddCharacter>, mint: Pubkey, _bump: u8) -> Result<()> {
        let character = &mut ctx.accounts.character;
        character.mint = mint;

        Ok(())
    }

    pub fn add_forge_recipe(
        ctx: Context<AddForgeRecipe>,
        _bump: u8,
        id: u16,
        data: Recipe
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.forge_recipe;

        let config = &mut ctx.accounts.config;

        if id != config.forge_recipes + 1 {
            return Err(ArisingError::InvalidForgeRecipeID.into());
        }

        msg!("Adding forge recipe {} with id {}", data.name, id);

        recipe.recipe.id = id;
        recipe.recipe.name = data.name;
        recipe.recipe.description = data.description;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        recipe.recipe.available = false;

        config.forge_recipes += 1;

        Ok(())
    }

    pub fn update_forge_recipe_availability(
        ctx: Context<UpdateForgeRecipe>,
        available: bool
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.forge_recipe;

        msg!("Updating forge recipe id {} availability to {}", recipe.recipe.id, available);

        recipe.recipe.available = available;

        Ok(())
    }

    pub fn update_forge_recipe(ctx: Context<UpdateForgeRecipe>, data: Recipe) -> Result<()> {
        let recipe = &mut ctx.accounts.forge_recipe;

        msg!("Updating forge recipe id {}", recipe.recipe.id);

        recipe.recipe.name = data.name;
        recipe.recipe.description = data.description;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        recipe.recipe.available = false;

        Ok(())
    }

    pub fn add_craft_recipe(
        ctx: Context<AddCraftRecipe>,
        _bump: u8,
        id: u16,
        data: Recipe
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.craft_recipe;

        let config = &mut ctx.accounts.config;

        if id != config.craft_recipes + 1 {
            return Err(ArisingError::InvalidCraftRecipeID.into());
        }

        msg!("Adding craft recipe {} with id {}", data.name, id);

        recipe.recipe.id = id;
        recipe.recipe.name = data.name;
        recipe.recipe.description = data.description;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        recipe.recipe.available = false;

        config.craft_recipes += 1;

        Ok(())
    }

    pub fn update_craft_recipe_availability(
        ctx: Context<UpdateCraftRecipe>,
        available: bool
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.craft_recipe;

        msg!("Updating craft recipe id {} availability to {}", recipe.recipe.id, available);

        recipe.recipe.available = available;

        Ok(())
    }

    pub fn update_craft_recipe(ctx: Context<UpdateCraftRecipe>, data: Recipe) -> Result<()> {
        let recipe = &mut ctx.accounts.craft_recipe;

        msg!("Updating craft recipe id {}", recipe.recipe.id);

        recipe.recipe.name = data.name;
        recipe.recipe.description = data.description;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        recipe.recipe.available = false;

        Ok(())
    }

    pub fn add_upgrade_recipe(
        ctx: Context<AddUpgradeRecipe>,
        _bump: u8,
        id: u16,
        data: Recipe
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.upgrade_recipe;

        let config = &mut ctx.accounts.config;

        if id != config.upgrade_recipes + 1 {
            return Err(ArisingError::InvalidUpgradeRecipeID.into());
        }

        msg!("Adding upgrade recipe {} with id {}", data.name, id);

        recipe.recipe.id = id;
        recipe.recipe.name = data.name;
        recipe.recipe.description = data.description;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        recipe.recipe.available = false;

        config.upgrade_recipes += 1;

        Ok(())
    }

    pub fn update_upgrade_recipe_availability(
        ctx: Context<UpdateUpgradeRecipe>,
        available: bool
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.upgrade_recipe;

        msg!("Updating upgrade recipe id {} availability to {}", recipe.recipe.id, available);

        recipe.recipe.available = available;

        Ok(())
    }

    pub fn update_upgrade_recipe(ctx: Context<UpdateUpgradeRecipe>, data: Recipe) -> Result<()> {
        let recipe = &mut ctx.accounts.upgrade_recipe;

        msg!("Updating upgrade recipe id {}", recipe.recipe.id);

        recipe.recipe.name = data.name;
        recipe.recipe.description = data.description;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        recipe.recipe.available = false;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetPause<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [CONFIG_PREFIX.as_bytes(), authority.key.as_ref()],
        bump,
        space = CONFIG_ACCOUNT_SIZE
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}