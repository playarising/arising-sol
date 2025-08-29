import { Program, Provider, Wallet } from '@project-serum/anchor'
import { PublicKey } from '@solana/web3.js'
import { Arising } from '../target/types/arising'
import { PROGRAM_ADDRESS as TOKEN_METADATA_PROGRAM } from '@metaplex-foundation/mpl-token-metadata'
import {
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token'

import { toAnchorFriendlyID } from './common'
import { Recipe } from './recipes'
import { QuestData } from './quests'

const CONFIG_PREFIX = 'arising_config_account'
const FORGE_RECIPE_PREFIX = 'arising_forge_recipe'
const CRAFT_RECIPE_PREFIX = 'arsing_craft'
const QUEST_PREFIX = 'arising_quest'

const METADATA_PREFIX = 'metadata'
const MASTER_EDITION_PREFIX = 'edition'

const CHARACTER_PREFIX = 'arising_character_account'
const CHARACTER_MATERIAL_PREFIX = 'arising_character_materials_account'
const CHARACTER_SLOTS_PREFIX = 'arising_character_slots_account'
const CHARACTER_EQUIPMENT_PREFIX = 'arising_character_equipment_account'

export const TOKEN_METADATA_PROGRAM_ID = new PublicKey(TOKEN_METADATA_PROGRAM)

// Returns the program main config account
export const getProgramConfigAccount = async (
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(CONFIG_PREFIX)],
        program.programId
    )

    return { account, bump }
}

// Returns the program character account from a mint
export const getProgramCharacterAccount = async (
    mint: PublicKey,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(CHARACTER_PREFIX), mint.toBuffer()],
        program.programId
    )

    return { account, bump }
}

// Returns the program character material account from a mint
export const getProgramCharacterMaterialsAccount = async (
    mint: PublicKey,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(CHARACTER_MATERIAL_PREFIX), mint.toBuffer()],
        program.programId
    )

    return { account, bump }
}

// Returns the program character slots account from a mint
export const getProgramCharacterSlotsAccount = async (
    mint: PublicKey,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(CHARACTER_SLOTS_PREFIX), mint.toBuffer()],
        program.programId
    )

    return { account, bump }
}

// Returns the program character equipment account from a mint
export const getProgramCharacterEquipmentAccount = async (
    mint: PublicKey,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(CHARACTER_EQUIPMENT_PREFIX), mint.toBuffer()],
        program.programId
    )

    return { account, bump }
}

// Returns the program forge recipe account from a recipe ID
export const getProgramForgeRecipeAccount = async (
    recipe: Recipe,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(FORGE_RECIPE_PREFIX), toAnchorFriendlyID(recipe.id)],
        program.programId
    )

    return { account, bump }
}

// Returns the program craft recipe account from a recipe ID
export const getProgramCraftRecipeAccount = async (
    recipe: Recipe,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(CRAFT_RECIPE_PREFIX), toAnchorFriendlyID(recipe.id)],
        program.programId
    )

    return { account, bump }
}

// Returns the program quest account from a quest ID
export const getProgramQuestAccount = async (
    quest: QuestData,
    program: Program<Arising>
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(QUEST_PREFIX), toAnchorFriendlyID(quest.id)],
        program.programId
    )

    return { account, bump }
}

// Returns the master edition account from a mint
export const getMasterEditionAccount = async (
    mint: PublicKey
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [
            Buffer.from(METADATA_PREFIX),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from(MASTER_EDITION_PREFIX),
        ],
        TOKEN_METADATA_PROGRAM_ID
    )

    return { account, bump }
}

// Returns the wallet associated token account from a mint
export const getTokenWalletAccount = async (
    wallet: PublicKey,
    mint: PublicKey
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [wallet.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
        ASSOCIATED_TOKEN_PROGRAM_ID
    )
    return { account, bump }
}

// Returns the metadata account from a mint
export const getMetadataAccount = async (
    mint: PublicKey
): Promise<{ account: PublicKey; bump: number }> => {
    const [account, bump] = await PublicKey.findProgramAddress(
        [
            Buffer.from(METADATA_PREFIX),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
    )

    return { account, bump }
}
