mod config;
mod utils;
mod characters;
mod errors;
mod codex;
mod forge;
mod quests;

use anchor_lang::prelude::*;

use config::*;
use characters::*;
use forge::*;
use quests::*;
use codex::*;
use errors::*;
use utils::*;

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
        let character = &ctx.accounts.character;

        if !can_assign_points(character, &points) {
            return Err(CharacterError::NotEnoughAssignablePoints.into());
        }

        let mut_character = &mut ctx.accounts.character;

        mut_character.base_stats.might += points.might;
        mut_character.base_stats.speed += points.speed;
        mut_character.base_stats.intellect += points.intellect;
        mut_character.pool_stats.might += points.might;
        mut_character.pool_stats.speed += points.speed;
        mut_character.pool_stats.intellect += points.intellect;

        Ok(())
    }

    pub fn add_forge_recipe(
        ctx: Context<AddForgeRecipe>,
        _bump: u8,
        id: u32,
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

    pub fn add_quest(ctx: Context<AddQuest>, _bump: u8, id: u32, data: Quest) -> Result<()> {
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

    pub fn perform_refresh(ctx: Context<CharacterAccess>) -> Result<()> {
        let character = &ctx.accounts.character;
        let config = &ctx.accounts.config;

        if !can_refresh(config, character) {
            return Err(CharacterError::RefreshNotAvailable.into());
        }

        let mut_character = &mut ctx.accounts.character;

        refresh(mut_character);

        return Ok(());
    }

    pub fn perform_refresh_with_token(ctx: Context<CharacterAccess>) -> Result<()> {
        let character = &ctx.accounts.character;
        let config = &ctx.accounts.config;

        if !can_refresh_with_token(config, character) {
            return Err(CharacterError::RefreshNotAvailable.into());
        }

        let mut_character = &mut ctx.accounts.character;

        refresh_with_token(mut_character);

        return Ok(());
    }

    pub fn start_forge(ctx: Context<ForgeAccess>) -> Result<()> {
        let recipe = &ctx.accounts.forge_recipe;
        let character = &ctx.accounts.character;

        // Check if the forge recipe is available globally.
        if !recipe.recipe.available {
            return Err(ForgeError::NotAvailable.into());
        }

        // Check if the character is able to forge
        if !is_slot_available(&character.craft) {
            return Err(CharacterError::NotAbleToForgeRecipe.into());
        }

        // Check if the character has enough level for the recipe
        if recipe.recipe.level_required > character.level {
            return Err(CharacterError::NotEnoughLevel.into());
        }

        // Check if the character can consume points of the pool
        if !can_consume(character, &recipe.recipe.stats_required) {
            return Err(CharacterError::NotEnoughPoolPointsToConsume.into());
        }

        let materials = &recipe.recipe.materials;
        let amounts = &recipe.recipe.materials_amounts;
        let types = &recipe.recipe.materials_types;

        // Check if the character can consume the materials for the recipe
        if !has_enough_materials(character, materials, amounts, types) {
            return Err(CharacterError::NotEnoughResources.into());
        }

        let mut_character = &mut ctx.accounts.character;

        // Consume the recipe material
        consume_materials(mut_character, materials, amounts, types);

        let stats = &recipe.recipe.stats_required;

        // Consume the pool points
        consume_points(mut_character, stats);

        // Store the recipe information for claim later
        mut_character.forge.cooldown = now() + (recipe.recipe.cooldown as u64);
        mut_character.forge.last_task_id = recipe.recipe.id;
        mut_character.forge.last_task_claimed = false;

        Ok(())
    }

    pub fn claim_forge(ctx: Context<ForgeAccess>) -> Result<()> {
        let recipe = &ctx.accounts.forge_recipe;
        let character = &ctx.accounts.character;

        // Check if the character is able to claim the forge recipe
        if !is_slot_claimable(&character.forge) {
            return Err(CharacterError::NotAbleToClaimForgeRecipe.into());
        }

        let material = recipe.recipe.item_rewarded;
        let amount = recipe.recipe.item_rewarded_amount;
        let material_type = recipe.recipe.item_rewarded_type;

        let mut_character = &mut ctx.accounts.character;

        // Reward the character
        forge_reward(mut_character, material, amount, material_type);

        // Modify the character forge slot to be able to create another recipe
        mut_character.forge.last_task_claimed = true;

        Ok(())
    }

    pub fn start_quest(ctx: Context<QuestAccess>, seed: u64) -> Result<()> {
        let quest = &ctx.accounts.quest;
        let character = &ctx.accounts.character;

        if !quest.available {
            return Err(QuestError::NotAvailable.into());
        }

        // Check if the character is able to start the quest
        if !is_slot_available(&character.quest) {
            return Err(CharacterError::NotAbleToQuest.into());
        }

        // Check if the character has enough level for the quest
        if quest.level_required > character.level {
            return Err(CharacterError::NotEnoughLevel.into());
        }

        // Check if the character can consume points of the pool
        if !can_consume(character, &quest.stats_required) {
            return Err(CharacterError::NotEnoughPoolPointsToConsume.into());
        }

        let mut_character = &mut ctx.accounts.character;
        consume_points(mut_character, &quest.stats_required);

        if quest.quest_type == (QuestType::Raid as u16) {
            // TODO: add the seed for randomness.
        }

        mut_character.quest.cooldown = now() + (quest.cooldown as u64);
        mut_character.quest.last_task_id = quest.id;
        mut_character.quest.last_task_claimed = false;

        Ok(())
    }

    pub fn claim_quest(ctx: Context<QuestAccess>) -> Result<()> {
        let quest = &ctx.accounts.quest;
        let character = &ctx.accounts.character;

        // Check if the character is able to claim the craft recipe
        if !is_slot_claimable(&character.quest) {
            return Err(CharacterError::NotAbleToClaimQuest.into());
        }

        let mut_character = &mut ctx.accounts.character;
        let materials = &quest.materials_reward;
        let amounts = &quest.materials_amounts;

        if
            quest.quest_type == (QuestType::Farm as u16) ||
            quest.quest_type == (QuestType::Job as u16)
        {
            quest_rewards(mut_character, materials, amounts);
        } else {
            // TODO: perform fight and experience reward.
        }

        // Modify the character slot to be able to start another quest
        let mut_character = &mut ctx.accounts.character;
        mut_character.quest.last_task_claimed = true;

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
        seeds = [CONFIG_PREFIX.as_bytes()],
        bump,
        space = CONFIG_ACCOUNT_SIZE
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}