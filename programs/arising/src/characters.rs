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
pub fn refresh(config: &Account<Config>, character: &mut Account<Character>) -> Result<()> {
    if character.last_refresh + config.seconds_between_refreshes < now() {
        return Err(CharacterError::RefreshNotAvailable.into());
    }

    character.pool_stats.might = character.base_stats.might;
    character.pool_stats.speed = character.base_stats.speed;
    character.pool_stats.intellect = character.base_stats.intellect;

    return Ok(());
}

#[inline(always)]
pub fn refresh_with_token(
    config: &Account<Config>,
    character: &mut Account<Character>
) -> Result<()> {
    if character.last_refresh_with_refresher + config.seconds_between_paid_refreshes < now() {
        return Err(CharacterError::RefreshNotAvailable.into());
    }

    character.pool_stats.might = character.base_stats.might;
    character.pool_stats.speed = character.base_stats.speed;
    character.pool_stats.intellect = character.base_stats.intellect;

    // TODO burn token.

    return Ok(());
}

#[inline(always)]
pub fn get_character_assignable_points(character: &Account<Character>) -> u64 {
    return (6 + character.level).into();
}

pub fn consume_points(character: &mut Account<Character>, points: &BaseStats) -> Result<()> {
    if character.pool_stats.might < points.might {
        return Err(CharacterError::NotEnoughPoolPointsToConsume.into());
    }

    if character.pool_stats.speed < points.speed {
        return Err(CharacterError::NotEnoughPoolPointsToConsume.into());
    }

    if character.pool_stats.intellect < points.intellect {
        return Err(CharacterError::NotEnoughPoolPointsToConsume.into());
    }

    character.pool_stats.might -= points.might;
    character.pool_stats.speed -= points.speed;
    character.pool_stats.intellect -= points.intellect;

    return Ok(());
}

#[inline(always)]
pub fn sacrifice_points(character: &mut Account<Character>, points: BaseStats) -> Result<()> {
    if character.base_stats.might < points.might {
        return Err(CharacterError::NotEnoughBasePointsToSacrifice.into());
    }

    if character.base_stats.speed < points.speed {
        return Err(CharacterError::NotEnoughBasePointsToSacrifice.into());
    }

    if character.base_stats.intellect < points.intellect {
        return Err(CharacterError::NotEnoughBasePointsToSacrifice.into());
    }

    character.base_stats.might -= points.might;
    character.base_stats.speed -= points.speed;
    character.base_stats.intellect -= points.intellect;

    let sum = points.might + points.speed + points.intellect;

    character.sacrificed_points += sum;

    return Ok(());
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
    64 + // last_recipe
    1; // last_recipe_claimed

/// The struct for slots used for character actions.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterSlot {
    pub cooldown: u64,
    pub last_recipe: u64,
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

/// The size of the character raw materials data in bytes.
pub const CHARACTER_RAW_MATERIALS_SIZE: usize =
    64 + // wood
    64 + // bones
    64 + // copper
    64 + // bronze
    64 + // stone
    64 + // iron
    64 + // leather
    64 + // cotton
    64 + // wool
    64 + // silk
    64 + // silver
    64 + // gold
    64 + // coal
    64 + // cobalt
    64 + // platinum
    64; // adamantine

/// The character raw materials struct.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterRawMaterials {
    pub wood: u64,
    pub bones: u64,
    pub copper: u64,
    pub bronze: u64,
    pub stone: u64,
    pub iron: u64,
    pub leather: u64,
    pub cotton: u64,
    pub wool: u64,
    pub silk: u64,
    pub silver: u64,
    pub gold: u64,
    pub coal: u64,
    pub cobalt: u64,
    pub platinum: u64,
    pub adamantine: u64,
}

/// The size of the character basic materials data in bytes.
pub const CHARACTER_BASIC_MATERIALS_SIZE: usize =
    64 + // wood_plank
    64 + // ironstone
    64 + // wool_fabric
    64 + // hardened_leader
    64 + // cotton_fabric
    64 + // silk_fabric
    64 + // copper_bar
    64 + // bronze_bar
    64 + // iron_bar
    64 + // silver_bar
    64 + // gold_bar
    64 + // steel_bar
    64 + // cobalt_bar
    64 + // platinum_bar
    64; // adamantine_bar

/// The character basic materials struct.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterBasicMaterials {
    pub wood_plank: u64,
    pub ironstone: u64,
    pub wool_fabric: u64,
    pub hardened_leader: u64,
    pub cotton_fabric: u64,
    pub silk_fabric: u64,
    pub copper_bar: u64,
    pub bronze_bar: u64,
    pub iron_bar: u64,
    pub silver_bar: u64,
    pub gold_bar: u64,
    pub steel_bar: u64,
    pub cobalt_bar: u64,
    pub platinum_bar: u64,
    pub adamantine_bar: u64,
}

/// The size of the character metadata in bytes.
pub const CHARACTER_ACCOUNT_SIZE: usize =
    8 + // discriminator
    64 + // level
    32 + // mint
    BASE_STATS_SIZE + // base_stats
    BASE_STATS_SIZE + // pool_stats
    64 + // experience
    64 + // last_refresh
    64 + // last_refresh_with_refresher
    64 + // sacrificed_points
    CHARACTER_SLOT_SIZE + // forge
    CHARACTER_SLOT_SIZE + // craft
    CHARACTER_SLOT_SIZE + // craft_upgrades
    CHARACTER_EQUIPMENT_SIZE + // equipment
    CHARACTER_RAW_MATERIALS_SIZE + // raw_materials
    CHARACTER_BASIC_MATERIALS_SIZE; // basic_materials

/// The full metadata information for an Arising character.
#[account]
pub struct Character {
    pub level: u64,
    pub mint: Pubkey,
    pub base_stats: BaseStats,
    pub pool_stats: BaseStats,
    pub experience: u64,
    pub last_refresh: u64,
    pub last_refresh_with_refresher: u64,
    pub sacrificed_points: u64,
    pub forge: CharacterSlot,
    pub craft: CharacterSlot,
    pub upgrade: CharacterSlot,
    pub equipment: CharacterEquipment,
    pub raw_materials: CharacterRawMaterials,
    pub basic_materials: CharacterBasicMaterials,
}