use anchor_lang::prelude::*;

pub const CONFIG_PREFIX: &str = "arising_config_account";

/// Config account bytes size.
pub const CONFIG_ACCOUNT_SIZE: usize =
    8 + // discriminator
    1 + // paused
    1 + // initialized
    32 + // authority
    32 + // seconds_between_refreshes
    32 + // seconds_between_paid_refreshes
    32 + // max_characters
    8 + // experience_multiplier
    16 + // forge_recipes
    16 + // craft_recipes
    16; // upgrade_recipes

/// Arising program config settings.
#[account]
#[derive(Default)]
pub struct Config {
    pub paused: bool,
    pub initialized: bool,
    pub authority: Pubkey,
    pub seconds_between_refreshes: u32,
    pub seconds_between_paid_refreshes: u32,
    pub max_characters: u32,
    pub experience_multiplier: u8,
    pub forge_recipes: u64,
    pub craft_recipes: u64,
    pub upgrade_recipes: u64,
}