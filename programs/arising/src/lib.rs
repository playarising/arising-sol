mod config;
mod characters;
mod errors;
mod codex;
mod recipes;

use anchor_lang::prelude::*;

use config::*;
use characters::*;
use recipes::*;
use errors::*;

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
        let forge_recipe = &mut ctx.accounts.forge_recipe;

        let config = &mut ctx.accounts.config;

        if id != config.forge_recipes + 1 {
            return Err(ArisingError::InvalidForgeRecipeID.into());
        }

        msg!("Adding forge recipe {} with id {}", data.name, id);

        forge_recipe.recipe.id = id;
        forge_recipe.recipe.name = data.name;
        forge_recipe.recipe.description = data.description;
        forge_recipe.recipe.materials = data.materials;
        forge_recipe.recipe.materials_amounts = data.materials_amounts;
        forge_recipe.recipe.stats_required = data.stats_required;
        forge_recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        forge_recipe.recipe.cooldown = data.cooldown;
        forge_recipe.recipe.level_required = data.level_required;
        forge_recipe.recipe.item_rewarded = data.item_rewarded;
        forge_recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        forge_recipe.recipe.available = false;

        config.forge_recipes += 1;

        Ok(())
    }

    pub fn update_forge_recipe_availability(
        ctx: Context<UpdateForgeRecipeAvailability>,
        available: bool
    ) -> Result<()> {
        let forge_recipe = &mut ctx.accounts.forge_recipe;

        msg!("Updating forge recipe id {} availability to {}", forge_recipe.recipe.id, available);

        forge_recipe.recipe.available = available;

        Ok(())
    }

    pub fn update_forge_recipe(ctx: Context<UpdateForgeRecipe>, data: Recipe) -> Result<()> {
        let forge_recipe = &mut ctx.accounts.forge_recipe;

        msg!("Updating forge recipe id {}", forge_recipe.recipe.id);

        forge_recipe.recipe.name = data.name;
        forge_recipe.recipe.description = data.description;
        forge_recipe.recipe.materials = data.materials;
        forge_recipe.recipe.materials_amounts = data.materials_amounts;
        forge_recipe.recipe.stats_required = data.stats_required;
        forge_recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        forge_recipe.recipe.cooldown = data.cooldown;
        forge_recipe.recipe.level_required = data.level_required;
        forge_recipe.recipe.item_rewarded = data.item_rewarded;
        forge_recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;
        forge_recipe.recipe.available = false;

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