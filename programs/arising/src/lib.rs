use anchor_lang::prelude::*;

declare_id!("EbYrmqgXkZsCxCP6tqWMCUTRcy1g9Q2asv3aeGDmBi1w");

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
        Ok(())
    }

    pub fn set_paused(ctx: Context<SetPause>, paused: bool) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.paused = paused;
        msg!("Changing pause status to {}", paused);
        Ok(())
    }

    pub fn add_character(ctx: Context<AddCharacter>) -> Result<()> {
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
pub struct AddCharacter<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ErrorCode::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub mint: Account<'info, CharacterMint>,

    #[account(
        init,
        payer = payer,
        seeds = [
            CHARACTER_PREFIX.as_bytes(),
            mint.to_account_info().key.as_ref(),
            payer.key.as_ref(),
        ],
        bump,
        space = CONFIG_ACCOUNT_SIZE
    )]
    pub character: Account<'info, CharacterData>,
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

#[account]
#[derive(Default)]
pub struct CharacterMint {
    mint: Pubkey,
}

pub const BASE_STATS_SIZE: usize =
    32 + // might
    32 + // speed
    32; // intellect

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BaseStats {
    might: u32,
    speed: u32,
    intellect: u32,
}

pub const CHARACTER_SLOT_SIZE: usize =
    32 + // cooldown
    32 + // last_recipe
    1; // last_recipe_claimed

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CharacterSlot {
    cooldown: u32,
    last_recipe: u32,
    last_recipe_claimed: bool,
}

pub const EQUIPMENT_SLOT_SIZE: usize =
    32 + // id
    1; // equiped

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EquipmentSlot {
    mint: Pubkey,
    equiped: bool,
}

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

#[account]
pub struct CharacterData {
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

pub const CONFIG_ACCOUNT_SIZE: usize =
    8 + // discriminator
    1 + // paused
    1 + // initialized
    32; // authority

#[account]
#[derive(Default)]
pub struct Config {
    paused: bool,
    initialized: bool,
    authority: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Authority is not the program authority")]
    InvalidAuthority,
}