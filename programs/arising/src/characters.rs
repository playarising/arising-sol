use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::codex::*;
use crate::config::*;
use crate::errors::*;
use crate::utils::*;

const CHARACTER_PREFIX: &str = "arising_character_account";
const CHARACTER_MATERIAL_PREFIX: &str = "arising_character_materials_account";
const CHARACTER_SLOTS_PREFIX: &str = "arising_character_slots_account";
const CHARACTER_EQUIPMENT_PREFIX: &str = "arising_character_equipment_account";

#[inline(always)]
pub fn is_mint_owner(
    mint: Pubkey,
    check_owner: Pubkey,
    owner_token_account: &Account<TokenAccount>,
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

    character.last_refresh = now();

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

    character.last_refresh_with_refresher = now();

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
pub fn consume_materials(
    character_materials: &mut Account<CharacterMaterials>,
    materials: &[u32; 10],
    amounts: &[u32; 10],
    types: &[u16; 10],
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
                character_materials.basic[(material - 1) as usize] -= amount;
            }

            if material_type == (ResourceType::Raw as u16) {
                character_materials.raw[(material - 1) as usize] -= amount;
            }
        }

        i += 1;
    }
}

#[inline(always)]
pub fn has_enough_materials(
    character_materials: &mut Account<CharacterMaterials>,
    materials: &[u32; 10],
    amounts: &[u32; 10],
    types: &[u16; 10],
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
                material_amount = character_materials.basic[(material as usize) - 1];
            }

            if material_type == (ResourceType::Raw as u16) {
                material_amount = character_materials.raw[(material as usize) - 1];
            }

            if amount_required > material_amount {
                return false;
            }
        }

        i += 1;
    }

    return true;
}

#[inline(always)]
pub fn add_experience(character: &mut Account<Character>, experience: u64) {
    character.experience += experience;
    character.level = get_level(character.experience);

    return;
}

#[inline(always)]
pub fn is_slot_available(slot: &CharacterSlot) -> bool {
    if slot.cooldown == 0 {
        return true;
    }

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
    pub character_materials: Account<'info, CharacterMaterials>,

    #[account(mut)]
    pub character_slots: Account<'info, CharacterSlots>,

    #[account(mut)]
    pub character_equipment: Account<'info, CharacterEquipment>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

pub const CHARACTER_ACCOUNT_SIZE: usize = 8 + // discriminator
    16 + // level
    32 + // mint
    BASE_STATS_SIZE + // base_stats
    BASE_STATS_SIZE + // pool_stats
    64 + // experience
    64 + // last_refresh
    64 + // last_refresh_with_refresher
    32; // sacrificed_points

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
}

pub const CHARACTER_MATERIALS_ACCOUNT_SIZE: usize = 8 + // discriminator
    1600 + // basic
    1600; // raw

#[account]
pub struct CharacterMaterials {
    pub basic: [u32; 20],
    pub raw: [u32; 20],
}

pub const CHARACTER_SLOT_SIZE: usize = 64 + // cooldown
    32 + // last_task_id
    1; // last_task_claimed

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterSlot {
    pub cooldown: u64,
    pub last_task_id: u32,
    pub last_task_claimed: bool,
}

pub const CHARACTER_SLOTS_ACCOUNT_SIZE: usize = 8 + // discriminator
    CHARACTER_SLOT_SIZE + // forge
    CHARACTER_SLOT_SIZE + // craft
    CHARACTER_SLOT_SIZE + // quest
    CHARACTER_SLOT_SIZE; // upgrade

#[account]
pub struct CharacterSlots {
    pub forge: CharacterSlot,
    pub craft: CharacterSlot,
    pub quest: CharacterSlot,
    pub upgrade: CharacterSlot,
}

/// The character informationsize in bytes.
pub const CHARACTER_EQUIPMENT_SIZE: usize = 32 + // helmer
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

#[account]
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
