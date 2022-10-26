import {
    BaseAttributes,
    BaseStats,
    EmptyAttributes,
    EmptyBaseStats,
} from './stats'

export enum ITEM {
    BONE_DAGGER = 1,
    BONE_HAMMER,
    BONE_AXE,
}

export enum ITEM_TYPE {
    ONE_HANDED = 1,
    TWO_HANDED,
    HELMET,
    SHOULDER_GUARD,
    ARM_GUARD,
    HAND,
    RING,
    NECKLACE,
    CHEST,
    LEG,
    BELT,
    FEET,
    CAPE,
}

interface ItemData {
    id: ITEM
    type: ITEM_TYPE
    additions: { stats: BaseStats; attributes: BaseAttributes }
    reductions: { stats: BaseStats; attributes: BaseAttributes }
    levelRequired: number
}

export const ITEMS: {
    [k in ITEM]: ItemData
} = {
    [ITEM.BONE_DAGGER]: {
        id: ITEM.BONE_DAGGER,
        type: ITEM_TYPE.ONE_HANDED,
        additions: {
            stats: EmptyBaseStats,
            attributes: {
                atk: 1,
                def: 0,
                range: 1,
                magAtk: 0,
                magDef: 0,
                rate: 1,
            },
        },
        reductions: { stats: EmptyBaseStats, attributes: EmptyAttributes },
        levelRequired: 0,
    },
    [ITEM.BONE_HAMMER]: {
        id: ITEM.BONE_HAMMER,
        type: ITEM_TYPE.ONE_HANDED,
        additions: {
            stats: { might: 1, speed: 0, intellect: 0 },
            attributes: EmptyAttributes,
        },
        reductions: { stats: EmptyBaseStats, attributes: EmptyAttributes },
        levelRequired: 3,
    },
    [ITEM.BONE_AXE]: {
        id: ITEM.BONE_AXE,
        type: ITEM_TYPE.ONE_HANDED,
        additions: {
            stats: { might: 0, speed: 1, intellect: 0 },
            attributes: EmptyAttributes,
        },
        reductions: { stats: EmptyBaseStats, attributes: EmptyAttributes },
        levelRequired: 3,
    },
}
