import { RAW_MATERIALS } from './raw_materials'
import {
    AnchorFriendlyBaseAttributes,
    AnchorFriendlyBaseStats,
    BaseAttributes,
    BaseStats,
    EmptyAttributes,
    EmptyBaseStats,
    toAnchorFriendlyBaseAttributes,
    toAnchorFriendlyBaseStats,
    toNormalBaseAttributes,
    toNormalBaseStats,
} from './stats'
import * as anchor from '@project-serum/anchor'

export enum QUESTS {
    BEG = 1,
    FIND_WOOD,
    FIND_STONE,
    RAT_HUNTING,
}

export enum QUEST_TYPE {
    JOB = 1,
    FARM,
    RAID,
}

interface AnchorFriendlyQuest {
    id: anchor.BN
    name: string
    description: string
    questType: anchor.BN
    statsRequired: AnchorFriendlyBaseStats
    cooldown: anchor.BN
    levelRequired: anchor.BN
    materialsReward: anchor.BN[]
    materialsAmounts: anchor.BN[]
    mobExperience: anchor.BN
    mobLevel: anchor.BN
    mobBaseStats: AnchorFriendlyBaseStats
    mobBaseAttributes: AnchorFriendlyBaseAttributes
}

export interface QuestData {
    id: QUESTS
    name: string
    description: string
    questType: QUEST_TYPE
    statsRequired: BaseStats
    cooldown: number
    levelRequired: number
    materialsReward: RAW_MATERIALS[]
    materialsAmounts: number[]
    mobExperience: number
    mobLevel: number
    mobBaseStats: BaseStats
    mobBaseAttributes: BaseAttributes
}

export function toAnchorFriendlyQuest(quest: QuestData): AnchorFriendlyQuest {
    return {
        id: new anchor.BN(quest.id),
        name: quest.name,
        description: quest.description,
        questType: new anchor.BN(quest.questType),
        statsRequired: toAnchorFriendlyBaseStats(quest.statsRequired),
        cooldown: new anchor.BN(quest.cooldown),
        levelRequired: new anchor.BN(quest.levelRequired),
        materialsReward: quest.materialsReward.map(
            (material) => new anchor.BN(material)
        ),
        materialsAmounts: quest.materialsAmounts.map(
            (amount) => new anchor.BN(amount)
        ),
        mobExperience: new anchor.BN(quest.mobExperience),
        mobLevel: new anchor.BN(quest.mobLevel),
        mobBaseStats: toAnchorFriendlyBaseStats(quest.mobBaseStats),
        mobBaseAttributes: toAnchorFriendlyBaseAttributes(
            quest.mobBaseAttributes
        ),
    }
}

export function toNormalQuest(quest: AnchorFriendlyQuest): QuestData {
    return {
        id: quest.id.toNumber(),
        name: quest.name,
        description: quest.description,
        questType: quest.questType.toNumber(),
        statsRequired: toNormalBaseStats(quest.statsRequired),
        cooldown: quest.cooldown.toNumber(),
        levelRequired: quest.levelRequired.toNumber(),
        materialsReward: quest.materialsReward
            .map((material) => material.toNumber())
            .filter((material) => material !== 0),
        materialsAmounts: quest.materialsAmounts
            .map((amount) => amount.toNumber())
            .filter((amount) => amount !== 0),
        mobExperience: quest.mobExperience.toNumber(),
        mobLevel: quest.mobLevel.toNumber(),
        mobBaseStats: toNormalBaseStats(quest.mobBaseStats),
        mobBaseAttributes: toNormalBaseAttributes(quest.mobBaseAttributes),
    }
}

export const QUESTS_DATA: {
    [k in QUESTS]: QuestData
} = {
    [QUESTS.BEG]: {
        id: QUESTS.BEG,
        name: 'Beg',
        description: 'Look for a place to beg some money and spend your time',
        questType: QUEST_TYPE.JOB,
        statsRequired: { might: 0, speed: 1, intellect: 1 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: [RAW_MATERIALS.GOLD],
        materialsAmounts: [5],
        mobExperience: 0,
        mobLevel: 0,
        mobBaseStats: EmptyBaseStats,
        mobBaseAttributes: EmptyAttributes,
    },
    [QUESTS.FIND_WOOD]: {
        id: QUESTS.FIND_WOOD,
        name: 'Find Wood',
        description: 'Walk to the forest and recollect the wood branches',
        questType: QUEST_TYPE.FARM,
        statsRequired: { might: 1, speed: 1, intellect: 0 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: [RAW_MATERIALS.WOOD],
        materialsAmounts: [1],
        mobExperience: 0,
        mobLevel: 0,
        mobBaseStats: EmptyBaseStats,
        mobBaseAttributes: EmptyAttributes,
    },
    [QUESTS.FIND_STONE]: {
        id: QUESTS.FIND_STONE,
        name: 'Find Stone',
        description: 'Walt to the roads and recollect some stones',
        questType: QUEST_TYPE.FARM,
        statsRequired: { might: 1, speed: 1, intellect: 0 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: [RAW_MATERIALS.STONE],
        materialsAmounts: [1],
        mobExperience: 0,
        mobLevel: 0,
        mobBaseStats: EmptyBaseStats,
        mobBaseAttributes: EmptyAttributes,
    },
    [QUESTS.RAT_HUNTING]: {
        id: QUESTS.RAT_HUNTING,
        name: 'Rat Hunting',
        description: 'Look through the sewers for rats to kill',
        questType: QUEST_TYPE.RAID,
        statsRequired: { might: 1, speed: 1, intellect: 1 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: [RAW_MATERIALS.BONES],
        materialsAmounts: [1],
        mobExperience: 10,
        mobLevel: 0,
        mobBaseStats: { might: 1, speed: 1, intellect: 1 },
        mobBaseAttributes: {
            atk: 1,
            def: 1,
            range: 1,
            magAtk: 0,
            magDef: 0,
            rate: 1,
        },
    },
}
