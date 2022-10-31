use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;
use crate::utils::*;

const CHARACTER_PREFIX: &str = "arising_character_account";

#[inline(always)]
pub fn is_mint_owner(
    mint: Pubkey,
    check_owner: Pubkey,
    owner_token_account: &Account<TokenAccount>
) -> bool {
    if mint != owner_token_account.mint {
        msg!("is_mint_owner: token mint doesn't match");
        return false;
    }

    if check_owner != owner_token_account.owner {
        msg!("is_mint_owner: owner_token_account owner doesn't match");
        return false;
    }

    if owner_token_account.amount < 1 {
        msg!("is_mint_owner: not enough amount of tokens to be owner");
        return false;
    }

    return true;
}

#[inline(always)]
pub fn can_refresh(config: &Account<Config>, character: &Account<Character>) -> bool {
    return character.last_refresh + config.seconds_between_refreshes < now();
}

#[inline(always)]
pub fn refresh(character: &mut Account<Character>) {
    character.pool_stats.might = character.base_stats.might;
    character.pool_stats.speed = character.base_stats.speed;
    character.pool_stats.intellect = character.base_stats.intellect;

    return;
}

#[inline(always)]
pub fn can_refresh_with_token(config: &Account<Config>, character: &Account<Character>) -> bool {
    return character.last_refresh_with_refresher + config.seconds_between_paid_refreshes < now();
}

#[inline(always)]
pub fn refresh_with_token(character: &mut Account<Character>) {
    character.pool_stats.might = character.base_stats.might;
    character.pool_stats.speed = character.base_stats.speed;
    character.pool_stats.intellect = character.base_stats.intellect;

    // TODO burn token.
}

#[inline(always)]
pub fn can_assign_points(character: &Account<Character>, points: &BaseStats) -> bool {
    let sum = points.might + points.speed + points.intellect;
    let available = get_character_assignable_points(character);
    return sum <= available;
}

#[inline(always)]
pub fn get_character_assignable_points(character: &Account<Character>) -> u16 {
    return 6 + character.level;
}

pub fn can_consume(character: &Account<Character>, points: &BaseStats) -> bool {
    if character.pool_stats.might < points.might {
        return false;
    }

    if character.pool_stats.speed < points.speed {
        return false;
    }

    if character.pool_stats.intellect < points.intellect {
        return false;
    }

    return true;
}

pub fn consume_points(character: &mut Account<Character>, points: &BaseStats) {
    character.pool_stats.might -= points.might;
    character.pool_stats.speed -= points.speed;
    character.pool_stats.intellect -= points.intellect;

    return;
}

#[inline(always)]
pub fn can_sacrifice(character: &mut Account<Character>, points: BaseStats) -> bool {
    if character.base_stats.might < points.might {
        return false;
    }

    if character.base_stats.speed < points.speed {
        return false;
    }

    if character.base_stats.intellect < points.intellect {
        return false;
    }

    return true;
}

#[inline(always)]
pub fn sacrifice_points(character: &mut Account<Character>, points: BaseStats) {
    character.base_stats.might -= points.might;
    character.base_stats.speed -= points.speed;
    character.base_stats.intellect -= points.intellect;

    let sum = points.might + points.speed + points.intellect;

    character.sacrificed_points += sum as u32;

    return;
}

#[inline(always)]
pub fn add_experience(character: &mut Account<Character>, experience: u64) {
    character.experience += experience;
    character.level = get_level(character.experience);

    return;
}

#[inline(always)]
pub fn is_slot_available(slot: &CharacterSlot) -> bool {
    // Check if this is the first use of the slot.
    if slot.cooldown == 0 {
        return true;
    }

    // Check if the cooldown has passed and the slot has been claimed.
    return slot.cooldown <= now() && slot.last_task_claimed;
}

#[inline(always)]
pub fn is_slot_claimable(slot: &CharacterSlot) -> bool {
    return slot.cooldown <= now() && !slot.last_task_claimed && slot.last_task_id != 0;
}

#[derive(Accounts)]
#[instruction(mint: Pubkey, bump: u8)]
pub struct AddCharacter<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [CHARACTER_PREFIX.as_bytes(), &mint.to_bytes()],
        bump,
        space = CHARACTER_ACCOUNT_SIZE
    )]
    pub character: Account<'info, Character>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CharacterAccess<'info> {
    #[account(mut,
        constraint = is_mint_owner(character.mint, payer.key(), &character_token_account) @ ArisingError::InvalidOwner)]
    payer: Signer<'info>,

    #[account(mut)]
    pub character: Account<'info, Character>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CharacterAccessWithConfig<'info> {
    #[account(mut,
        constraint = is_mint_owner(character.mint, payer.key(), &character_token_account) @ ArisingError::InvalidOwner)]
    payer: Signer<'info>,

    #[account(mut)]
    pub character: Account<'info, Character>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

/// The size of the character struct for actions.
pub const CHARACTER_SLOT_SIZE: usize =
    64 + // cooldown
    32 + // last_task_id
    1; // last_task_claimed

/// The struct for slots used for character actions.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterSlot {
    pub cooldown: u64,
    pub last_task_id: u32,
    pub last_task_claimed: bool,
}

/// The character informationsize in bytes.
pub const CHARACTER_EQUIPMENT_SIZE: usize =
    32 + // helmer
    32 + // shoulder_guards
    32 + // arm_guards
    32 + // hands
    32 + // rings
    32 + // necklace
    32 + // chest
    32 + // legs
    32 + // belt
    32 + // feet
    32 + // cape
    32 + // left_hand
    32; // right_hand

/// The character equipment struct.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterEquipment {
    pub helmet: Pubkey,
    pub shoulder_guards: Pubkey,
    pub arm_guards: Pubkey,
    pub hands: Pubkey,
    pub rings: Pubkey,
    pub necklace: Pubkey,
    pub chest: Pubkey,
    pub legs: Pubkey,
    pub belt: Pubkey,
    pub feet: Pubkey,
    pub cape: Pubkey,
    pub left_hand: Pubkey,
    pub right_hand: Pubkey,
}

const CHARACTER_RESOURCES_SIZE: usize =
    544 + // basic
    544; // raw

/// The character equipment struct.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterResources {
    pub basic: [u32; 17],
    pub raw: [u32; 17],
}

/// The size of the character metadata in bytes.
pub const CHARACTER_ACCOUNT_SIZE: usize =
    8 + // discriminator
    16 + // level
    32 + // mint
    BASE_STATS_SIZE + // base_stats
    BASE_STATS_SIZE + // pool_stats
    64 + // experience
    64 + // last_refresh
    64 + // last_refresh_with_refresher
    32 + // sacrificed_points
    CHARACTER_SLOT_SIZE + // forge
    CHARACTER_SLOT_SIZE + // craft
    CHARACTER_SLOT_SIZE + // quest
    CHARACTER_EQUIPMENT_SIZE + // equipment
    CHARACTER_RESOURCES_SIZE; // resources

/// The full metadata information for an Arising character.
#[account]
pub struct Character {
    pub level: u16,
    pub mint: Pubkey,
    pub base_stats: BaseStats,
    pub pool_stats: BaseStats,
    pub experience: u64,
    pub last_refresh: u64,
    pub last_refresh_with_refresher: u64,
    pub sacrificed_points: u32,
    pub forge: CharacterSlot,
    pub craft: CharacterSlot,
    pub quest: CharacterSlot,
    pub equipment: CharacterEquipment,
    pub resources: CharacterResources,
}