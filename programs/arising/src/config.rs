use anchor_lang::prelude::*;

pub const CONFIG_PREFIX: &str = "arising_config_account";

/// Config account bytes size.
pub const CONFIG_ACCOUNT_SIZE: usize =
    8 + // discriminator
    1 + // paused
    1 + // initialized
    32 + // authority
    64 + // seconds_between_refreshes
    64 + // seconds_between_paid_refreshes
    64 + // max_characters
    64 + // experience_multiplier
    64 + // forge_recipes
    64 + // craft_recipes
    64; // upgrade_recipes

/// Arising program config settings.
#[account]
#[derive(Default)]
pub struct Config {
    pub paused: bool,
    pub initialized: bool,
    pub authority: Pubkey,
    pub seconds_between_refreshes: u64,
    pub seconds_between_paid_refreshes: u64,
    pub max_characters: u64,
    pub experience_multiplier: u64,
    pub forge_recipes: u64,
    pub craft_recipes: u64,
    pub upgrade_recipes: u64,
    pub quests: u64,
}