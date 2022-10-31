import { toFixedArray } from './common'
import { RAW_MATERIALS } from './raw_materials'
import {
    BaseAttributes,
    BaseStats,
    EmptyAttributes,
    EmptyBaseStats,
} from './stats'

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
    available: boolean
}

export const MockJobQuest = (): QuestData => {
    const baseQuest = QUESTS_DATA[QUESTS.BEG]

    baseQuest.cooldown = 2

    baseQuest.statsRequired = { might: 1, speed: 0, intellect: 0 }

    baseQuest.materialsReward = [
        RAW_MATERIALS.GOLD,
        RAW_MATERIALS.WOOD,
        RAW_MATERIALS.IRON,
        RAW_MATERIALS.SILVER,
        RAW_MATERIALS.BONES,
        RAW_MATERIALS.BRONZE,
        RAW_MATERIALS.LEATHER,
        RAW_MATERIALS.COTTON,
        RAW_MATERIALS.WOOL,
        RAW_MATERIALS.SILK,
    ]

    baseQuest.materialsAmounts = [
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ]

    return baseQuest
}

export const MockFarmQuest = (): QuestData => {
    const baseQuest = QUESTS_DATA[QUESTS.FIND_WOOD]
    baseQuest.cooldown = 2
    baseQuest.statsRequired = { might: 0, speed: 1, intellect: 0 }

    baseQuest.materialsReward = toFixedArray(10, [
        RAW_MATERIALS.WOOD,
        RAW_MATERIALS.BONES,
    ])
    baseQuest.materialsAmounts = toFixedArray(10, [100, 100])
    return baseQuest
}

export const MockRaidQuest = (): QuestData => {
    const baseQuest = QUESTS_DATA[QUESTS.FIND_STONE]
    baseQuest.cooldown = 2
    baseQuest.statsRequired = { might: 0, speed: 0, intellect: 1 }
    baseQuest.questType = QUEST_TYPE.RAID
    baseQuest.materialsReward = toFixedArray(10, [RAW_MATERIALS.BONES])
    baseQuest.materialsAmounts = toFixedArray(10, [50])
    baseQuest.mobExperience = 500
    baseQuest.mobLevel = 0
    baseQuest.mobBaseStats = { might: 1, speed: 1, intellect: 0 }
    baseQuest.mobBaseAttributes = {
        atk: 1,
        def: 1,
        range: 1,
        magAtk: 0,
        magDef: 0,
        rate: 1,
    }

    return baseQuest
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
        materialsReward: toFixedArray(10, [RAW_MATERIALS.GOLD]),
        materialsAmounts: toFixedArray(10, [5]),
        mobExperience: 0,
        mobLevel: 0,
        mobBaseStats: EmptyBaseStats,
        mobBaseAttributes: EmptyAttributes,
        available: false,
    },
    [QUESTS.FIND_WOOD]: {
        id: QUESTS.FIND_WOOD,
        name: 'Find Wood',
        description: 'Walk to the forest and recollect the wood branches',
        questType: QUEST_TYPE.FARM,
        statsRequired: { might: 1, speed: 1, intellect: 0 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: toFixedArray(10, [RAW_MATERIALS.WOOD]),
        materialsAmounts: toFixedArray(10, [1]),
        mobExperience: 0,
        mobLevel: 0,
        mobBaseStats: EmptyBaseStats,
        mobBaseAttributes: EmptyAttributes,
        available: false,
    },
    [QUESTS.FIND_STONE]: {
        id: QUESTS.FIND_STONE,
        name: 'Find Stone',
        description: 'Walt to the roads and recollect some stones',
        questType: QUEST_TYPE.FARM,
        statsRequired: { might: 1, speed: 1, intellect: 0 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: toFixedArray(10, [RAW_MATERIALS.STONE]),
        materialsAmounts: toFixedArray(10, [1]),
        mobExperience: 0,
        mobLevel: 0,
        mobBaseStats: EmptyBaseStats,
        mobBaseAttributes: EmptyAttributes,
        available: false,
    },
    [QUESTS.RAT_HUNTING]: {
        id: QUESTS.RAT_HUNTING,
        name: 'Rat Hunting',
        description: 'Look through the sewers for rats to kill',
        questType: QUEST_TYPE.RAID,
        statsRequired: { might: 1, speed: 1, intellect: 1 },
        cooldown: 600,
        levelRequired: 0,
        materialsReward: toFixedArray(10, [RAW_MATERIALS.BONES]),
        materialsAmounts: toFixedArray(10, [1]),
        mobExperience: 20,
        mobLevel: 0,
        mobBaseStats: { might: 1, speed: 1, intellect: 0 },
        mobBaseAttributes: {
            atk: 1,
            def: 1,
            range: 1,
            magAtk: 0,
            magDef: 0,
            rate: 1,
        },
        available: false,
    },
}
