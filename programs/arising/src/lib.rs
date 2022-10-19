pub mod checks;

use anchor_lang::prelude::*;

declare_id!("GT1koQQwD6ZV6bxciNSwC3YFDHiByySKZbQ2MQJF4GWp");

const CONFIG_PREFIX: &str = "arising_config_account";
const CHARACTER_PREFIX: &str = "arising_character_account";

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
}

#[derive(Accounts)]
pub struct SetPause<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ErrorCode::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,
}

#[derive(Accounts)]
#[instruction(mint: Pubkey, bump: u8)]
pub struct AddCharacter<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ErrorCode::InvalidAuthority)]
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

/// The size of the character stats.
pub const BASE_STATS_SIZE: usize =
    32 + // might
    32 + // speed
    32; // intellect

/// The struct for character stats.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BaseStats {
    might: u32,
    speed: u32,
    intellect: u32,
}

/// The size of the character struct for actions.
pub const CHARACTER_SLOT_SIZE: usize =
    32 + // cooldown
    32 + // last_recipe
    1; // last_recipe_claimed

/// The struct for slots used for character actions.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterSlot {
    cooldown: u32,
    last_recipe: u32,
    last_recipe_claimed: bool,
}

/// The size of the slot in bytes
pub const EQUIPMENT_SLOT_SIZE: usize =
    32 + // id
    1; // equiped

/// One slot for the character equipment.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EquipmentSlot {
    mint: Pubkey,
    equiped: bool,
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
    helmet: EquipmentSlot,
    shoulder_guards: EquipmentSlot,
    arm_guards: EquipmentSlot,
    hands: EquipmentSlot,
    rings: EquipmentSlot,
    necklace: EquipmentSlot,
    chest: EquipmentSlot,
    legs: EquipmentSlot,
    belt: EquipmentSlot,
    feet: EquipmentSlot,
    cape: EquipmentSlot,
    left_hand: EquipmentSlot,
    right_hand: EquipmentSlot,
}

/// The size of the character metadata in bytes.
pub const CHARACTER_ACCOUNT_SIZE: usize =
    8 + // discriminator
    32 + // level
    32 + // mint
    BASE_STATS_SIZE + // base_stats
    BASE_STATS_SIZE + // pool_stats
    32 + // experience
    32 + // last_refresh
    32 + // last_refresh_with_refresher
    32 + // sacrificed_points
    CHARACTER_SLOT_SIZE + // forge
    CHARACTER_SLOT_SIZE + // craft
    CHARACTER_SLOT_SIZE + // craft_upgrades
    CHARACTER_EQUIPMENT_SIZE; // equipment

/// The full metadata information for an Arising character.
#[account]
pub struct Character {
    level: u32,
    mint: Pubkey,
    base_stats: BaseStats,
    pool_stats: BaseStats,
    experience: u32,
    last_refresh: u32,
    last_refresh_with_refresher: u32,
    sacrificed_points: u32,
    forge: CharacterSlot,
    craft: CharacterSlot,
    craft_upgrades: CharacterSlot,
    equipment: CharacterEquipment,
}

/// Config account bytes size.
pub const CONFIG_ACCOUNT_SIZE: usize =
    8 + // discriminator
    1 + // paused
    1 + // initialized
    32 + // authority
    32 + // seconds_between_refreshes
    32 + // seconds_between_paid_refreshes
    32 + // max_characters
    32; // experience_multiplier

/// Arising program config settings.
#[account]
#[derive(Default)]
pub struct Config {
    paused: bool,
    initialized: bool,
    authority: Pubkey,
    seconds_between_refreshes: u32,
    seconds_between_paid_refreshes: u32,
    max_characters: u32,
    experience_multiplier: u32,
}

/// Program error codes.
#[error_code]
pub enum ErrorCode {
    #[msg("Authority is not the program authority.")]
    InvalidAuthority,
    #[msg("Payer is not owner of the token.")]
    InvalidOwner,
}