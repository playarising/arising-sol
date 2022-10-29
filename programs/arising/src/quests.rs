use anchor_lang::prelude::*;

use crate::config::*;
use crate::errors::*;
use crate::codex::*;

const QUESTS_PREFIX: &str = "arising_quest";

#[derive(Accounts)]
pub struct UpdateQuest<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority)]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub quest: Account<'info, Quest>,
}

#[derive(Accounts)]
#[instruction(bump: u8, id: u64)]
pub struct AddQuest<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.quests + 1) == id @ QuestError::InvalidID
    )]
    payer: Signer<'info>,

    #[account(mut)]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = payer,
        seeds = [QUESTS_PREFIX.as_bytes(), &id.to_le_bytes()],
        bump,
        space = QUEST_SIZE
    )]
    pub quest: Account<'info, Quest>,

    pub system_program: Program<'info, System>,
}

enum QuestType {
    JOB = 1,
    FARM,
    RAID,
}

/// The size of a quest.
pub const QUEST_SIZE: usize =
    8 + // discriminator
    64 + // id
    24 + // name
    24 + // description
    64 + // quest_type
    BASE_STATS_SIZE + // stats_required
    64 + // cooldown
    64 + // level_required
    640 + // materials_reward
    640 + // materials_amounts
    64 + // mob_experience
    64 + // mob_level
    BASE_STATS_SIZE + // mob_base_stats
    BASE_ATTRIBUTES_SIZE + // mob_base_attributes
    1; // available

/// The full metadata information for a quest.
#[account]
pub struct Quest {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub quest_type: u64,
    pub stats_required: BaseStats,
    pub cooldown: u64,
    pub level_required: u64,
    pub materials_reward: [u64; 10],
    pub materials_amounts: [u64; 10],
    pub mob_experience: u64,
    pub mob_level: u64,
    pub mob_base_stats: BaseStats,
    pub mob_base_attributes: BaseAttributes,
    pub available: bool,
}