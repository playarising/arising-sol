use anchor_lang::prelude::*;

declare_id!("EbYrmqgXkZsCxCP6tqWMCUTRcy1g9Q2asv3aeGDmBi1w");

const CONFIG_PREFIX: &str = "arising_config_account";

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
pub struct MintData {
    level: u32,
    mint: Pubkey,
}

pub const CONFIG_ACCOUNT_SIZE: usize =
    8 + // discriminator
    1 + // paused
    1 + // initialized
    32; // authority

#[account]
#[derive(Default)]
pub struct Config {
    // 1
    paused: bool,
    // 1
    initialized: bool,
    // 32
    authority: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Authority is not the program authority")]
    InvalidAuthority,
}