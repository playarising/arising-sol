use anchor_lang::prelude::*;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;

const CHARACTER_PREFIX: &str = "arising_character_account";

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

/// The size of the character struct for actions.
pub const CHARACTER_SLOT_SIZE: usize =
    64 + // cooldown
    16 + // last_recipe
    1; // last_recipe_claimed

/// The struct for slots used for character actions.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterSlot {
    pub cooldown: u64,
    pub last_recipe: u16,
    pub last_recipe_claimed: bool,
}

/// The size of the slot in bytes
pub const EQUIPMENT_SLOT_SIZE: usize =
    32 + // mint
    1; // equiped

/// One slot for the character equipment.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EquipmentSlot {
    pub mint: Pubkey,
    pub equiped: bool,
}

/// The character informationsize in bytes.
pub const CHARACTER_EQUIPMENT_SIZE: usize =
    EQUIPMENT_SLOT_SIZE + // helmer
    EQUIPMENT_SLOT_SIZE + // shoulder_guards
    EQUIPMENT_SLOT_SIZE + // arm_guards
    EQUIPMENT_SLOT_SIZE + // hands
    EQUIPMENT_SLOT_SIZE + // rings
    EQUIPMENT_SLOT_SIZE + // necklace
    EQUIPMENT_SLOT_SIZE + // chest
    EQUIPMENT_SLOT_SIZE + // legs
    EQUIPMENT_SLOT_SIZE + // belt
    EQUIPMENT_SLOT_SIZE + // feet
    EQUIPMENT_SLOT_SIZE + // cape
    EQUIPMENT_SLOT_SIZE + // left_hand
    EQUIPMENT_SLOT_SIZE; // right_hand

/// The character equipment struct.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterEquipment {
    pub helmet: EquipmentSlot,
    pub shoulder_guards: EquipmentSlot,
    pub arm_guards: EquipmentSlot,
    pub hands: EquipmentSlot,
    pub rings: EquipmentSlot,
    pub necklace: EquipmentSlot,
    pub chest: EquipmentSlot,
    pub legs: EquipmentSlot,
    pub belt: EquipmentSlot,
    pub feet: EquipmentSlot,
    pub cape: EquipmentSlot,
    pub left_hand: EquipmentSlot,
    pub right_hand: EquipmentSlot,
}

/// The size of the character metadata in bytes.
pub const CHARACTER_ACCOUNT_SIZE: usize =
    8 + // discriminator
    16 + // level
    32 + // mint
    BASE_STATS_SIZE + // base_stats
    BASE_STATS_SIZE + // pool_stats
    32 + // experience
    32 + // last_refresh
    32 + // last_refresh_with_refresher
    16 + // sacrificed_points
    CHARACTER_SLOT_SIZE + // forge
    CHARACTER_SLOT_SIZE + // craft
    CHARACTER_SLOT_SIZE + // craft_upgrades
    CHARACTER_EQUIPMENT_SIZE; // equipment

/// The full metadata information for an Arising character.
#[account]
pub struct Character {
    pub level: u16,
    pub mint: Pubkey,
    pub base_stats: BaseStats,
    pub pool_stats: BaseStats,
    pub experience: u32,
    pub last_refresh: u32,
    pub last_refresh_with_refresher: u32,
    pub sacrificed_points: u16,
    pub forge: CharacterSlot,
    pub craft: CharacterSlot,
    pub upgrade: CharacterSlot,
    pub equipment: CharacterEquipment,
}