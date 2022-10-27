mod config;
mod utils;
mod characters;
mod errors;
mod codex;
mod recipes;
mod quests;

use anchor_lang::prelude::*;

use config::*;
use characters::*;
use recipes::*;
use quests::*;
use codex::*;
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

    pub fn assign_stats_character(ctx: Context<CharacterAccess>, points: BaseStats) -> Result<()> {
        let sum = points.might + points.speed + points.intellect;

        if sum > get_character_assignable_points(&ctx.accounts.character) {
            return Err(ArisingError::InvalidAssignPoints.into());
        }

        let character = &mut ctx.accounts.character;

        character.base_stats.might += points.might;
        character.base_stats.speed += points.speed;
        character.base_stats.intellect += points.intellect;
        character.pool_stats.might += points.might;
        character.pool_stats.speed += points.speed;
        character.pool_stats.intellect += points.intellect;

        Ok(())
    }

    pub fn add_forge_recipe(
        ctx: Context<AddForgeRecipe>,
        _bump: u8,
        id: u64,
        data: Recipe
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.forge_recipe;

        let config = &mut ctx.accounts.config;

        msg!("Adding {} with id {}", data.name, id);

        recipe.recipe.id = id;
        recipe.recipe.name = data.name;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.materials_types = data.materials_types;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_type = data.item_rewarded_type;
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
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.materials_types = data.materials_types;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_type = data.item_rewarded_type;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;

        Ok(())
    }

    pub fn add_craft_recipe(
        ctx: Context<AddCraftRecipe>,
        _bump: u8,
        id: u64,
        data: Recipe
    ) -> Result<()> {
        let recipe = &mut ctx.accounts.craft_recipe;

        let config = &mut ctx.accounts.config;

        msg!("Adding craft recipe {} with id {}", data.name, id);

        recipe.recipe.id = id;
        recipe.recipe.name = data.name;
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.materials_types = data.materials_types;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_type = data.item_rewarded_type;
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
        recipe.recipe.materials = data.materials;
        recipe.recipe.materials_amounts = data.materials_amounts;
        recipe.recipe.materials_types = data.materials_types;
        recipe.recipe.stats_required = data.stats_required;
        recipe.recipe.stats_sacrificed = data.stats_sacrificed;
        recipe.recipe.cooldown = data.cooldown;
        recipe.recipe.level_required = data.level_required;
        recipe.recipe.item_rewarded = data.item_rewarded;
        recipe.recipe.item_rewarded_type = data.item_rewarded_type;
        recipe.recipe.item_rewarded_amount = data.item_rewarded_amount;

        Ok(())
    }

    pub fn add_quest(ctx: Context<AddQuest>, _bump: u8, id: u64, data: Quest) -> Result<()> {
        let quest = &mut ctx.accounts.quest;

        let config = &mut ctx.accounts.config;

        msg!("Adding quest {} with id {}", data.name, id);

        quest.id = id;
        quest.name = data.name;
        quest.description = data.description;
        quest.quest_type = data.quest_type;
        quest.stats_required = data.stats_required;
        quest.cooldown = data.cooldown;
        quest.level_required = data.level_required;
        quest.materials_reward = data.materials_reward;
        quest.materials_amounts = data.materials_amounts;
        quest.mob_experience = data.mob_experience;
        quest.mob_level = data.mob_level;
        quest.mob_base_stats = data.mob_base_stats;
        quest.mob_base_attributes = data.mob_base_attributes;
        quest.available = false;

        config.quests += 1;

        Ok(())
    }

    pub fn update_quest_availability(ctx: Context<UpdateQuest>, available: bool) -> Result<()> {
        let quest = &mut ctx.accounts.quest;

        msg!("Updating quest id {} availability to {}", quest.id, available);

        quest.available = available;

        Ok(())
    }

    pub fn update_quest(ctx: Context<UpdateQuest>, data: Quest) -> Result<()> {
        let quest = &mut ctx.accounts.quest;

        msg!("Updating quest id {}", quest.id);

        quest.name = data.name;
        quest.description = data.description;
        quest.quest_type = data.quest_type;
        quest.stats_required = data.stats_required;
        quest.cooldown = data.cooldown;
        quest.level_required = data.level_required;
        quest.materials_reward = data.materials_reward;
        quest.materials_amounts = data.materials_amounts;
        quest.mob_experience = data.mob_experience;
        quest.mob_level = data.mob_level;
        quest.mob_base_stats = data.mob_base_stats;
        quest.mob_base_attributes = data.mob_base_attributes;

        Ok(())
    }

    pub fn start_forge(ctx: Context<ForgeAccess>) -> Result<()> {
        Ok(())
    }

    pub fn claim_forge(ctx: Context<ForgeAccess>) -> Result<()> {
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