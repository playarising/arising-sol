use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::characters::*;
use crate::codex::*;
use crate::config::*;
use crate::errors::*;

const QUESTS_PREFIX: &str = "arising_quest";

#[inline(always)]
pub fn quest_rewards(
    character: &mut Account<Character>,
    materials: &[u32; 10],
    amounts: &[u32; 10],
) {
    let mut i: usize = 0;
    loop {
        if i >= 10 {
            break;
        }

        let material = materials[i];
        let amount = amounts[i];

        if material != 0 {
            character.raw[(material - 1) as usize] += amount;
        }

        i += 1;
    }
}

#[derive(Accounts)]
pub struct QuestAccess<'info> {
    #[account(mut,
        constraint = is_mint_owner(character.mint, payer.key(), &character_token_account) @ ArisingError::InvalidOwner)]
    payer: Signer<'info>,

    #[account(mut)]
    pub character: Account<'info, Character>,

    #[account(mut)]
    pub character_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub quest: Account<'info, Quest>,
}

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
#[instruction(bump: u8, id: u32)]
pub struct AddQuest<'info> {
    #[account(mut,
        constraint = payer.key() == config.authority @ ArisingError::InvalidAuthority,
        constraint = (config.quests + 1) == (id as u64) @ QuestError::InvalidID
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

pub enum QuestType {
    Job = 1,
    Farm,
    Raid,
}

/// The size of a quest.
pub const QUEST_SIZE: usize = 8 + // discriminator
    32 + // id
    24 + // name
    24 + // description
    16 + // quest_type
    BASE_STATS_SIZE + // stats_required
    32 + // cooldown
    16 + // level_required
    32 + // materials_reward
    32 + // materials_amounts
    32 + // mob_experience
    16 + // mob_level
    BASE_STATS_SIZE + // mob_base_stats
    BASE_ATTRIBUTES_SIZE + // mob_base_attributes
    1; // available

/// The full metadata information for a quest.
#[account]
pub struct Quest {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub quest_type: u16,
    pub stats_required: BaseStats,
    pub cooldown: u32,
    pub level_required: u16,
    pub materials_reward: [u32; 10],
    pub materials_amounts: [u32; 10],
    pub mob_experience: u32,
    pub mob_level: u16,
    pub mob_base_stats: BaseStats,
    pub mob_base_attributes: BaseAttributes,
    pub available: bool,
}
