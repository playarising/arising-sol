import { BASIC_MATERIAL } from './basic_materials'
import { RESOURCE_TYPE } from './common'
import { ITEM } from './items'
import { RAW_MATERIALS } from './raw_materials'
import {
    AnchorFriendlyBaseStats,
    BaseStats,
    EmptyBaseStats,
    toAnchorFriendlyBaseStats,
} from './stats'
import * as anchor from '@project-serum/anchor'

interface AnchorFriendlyRecipe {
    id: anchor.BN
    name: string
    materials: anchor.BN[]
    materialsAmounts: anchor.BN[]
    materialsTypes: anchor.BN[]
    statsRequired: AnchorFriendlyBaseStats
    statsSacrificed: AnchorFriendlyBaseStats
    cooldown: anchor.BN
    levelRequired: anchor.BN
    itemRewarded: anchor.BN
    itemRewardedType: anchor.BN
    itemRewardedAmount: anchor.BN
}

interface Recipe {
    id: number
    name: string
    materials: number[]
    materialsAmounts: number[]
    materialsTypes: RESOURCE_TYPE[]
    statsRequired: BaseStats
    statsSacrificed: BaseStats
    cooldown: number
    levelRequired: number
    itemRewarded: BASIC_MATERIAL | ITEM
    itemRewardedType: RESOURCE_TYPE
    itemRewardedAmount: number
}

export function toAnchorFriendlyRecipe(recipe: Recipe): AnchorFriendlyRecipe {
    return {
        id: new anchor.BN(recipe.id),
        name: recipe.name,
        materials: recipe.materials.map((material) => new anchor.BN(material)),
        materialsAmounts: recipe.materialsAmounts.map(
            (amount) => new anchor.BN(amount)
        ),
        materialsTypes: recipe.materialsTypes.map(
            (type) => new anchor.BN(type)
        ),
        statsRequired: toAnchorFriendlyBaseStats(recipe.statsRequired),
        statsSacrificed: toAnchorFriendlyBaseStats(recipe.statsSacrificed),
        cooldown: new anchor.BN(recipe.cooldown),
        levelRequired: new anchor.BN(recipe.levelRequired),
        itemRewarded: new anchor.BN(recipe.itemRewarded),
        itemRewardedType: new anchor.BN(recipe.itemRewardedType),
        itemRewardedAmount: new anchor.BN(recipe.itemRewardedAmount),
    }
}

export enum FORGE_RECIPE {
    WOOD_PLANK,
    IRONSTONE,
    WOOL_FABRIC,
    HARDENED_LEATHER,
    COTTON_FABRIC,
    SILK_FABRIC,
    COPPER_BAR,
    BRONZE_BAR,
    IRON_BAR,
    SILVER_BAR,
    GOLD_BAR,
    STEEL_BAR,
    COBALT_BAR,
    PLATINUM_BAR,
    ADAMANTINE_BAR,
}

export enum CRAFT_RECIPE {
    BONE_DAGGER,
    BONE_HAMMER,
    BONE_AXE,
}

export const FORGE_RECIPES_DATA: {
    [k in FORGE_RECIPE]: Recipe
} = {
    [FORGE_RECIPE.WOOD_PLANK]: {
        id: FORGE_RECIPE.WOOD_PLANK,
        name: 'Forge a Wood Plank',
        materials: [RAW_MATERIALS.WOOD, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [5, 20],
        statsRequired: { might: 0, speed: 1, intellect: 1 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 900,
        levelRequired: 0,
        itemRewarded: BASIC_MATERIAL.WOOD_PLANK,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.IRONSTONE]: {
        id: FORGE_RECIPE.IRONSTONE,
        name: 'Forge an Ironstone',
        materials: [
            RAW_MATERIALS.STONE,
            RAW_MATERIALS.IRON,
            RAW_MATERIALS.GOLD,
        ],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
        ],
        materialsAmounts: [10, 1, 25],
        statsRequired: { might: 0, speed: 1, intellect: 2 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 900,
        levelRequired: 5,
        itemRewarded: BASIC_MATERIAL.IRONSTONE,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.WOOL_FABRIC]: {
        id: FORGE_RECIPE.WOOL_FABRIC,
        name: 'Forge Wool Fabric',
        materials: [RAW_MATERIALS.WOOL, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [10, 30],
        statsRequired: { might: 0, speed: 2, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 1800,
        levelRequired: 10,
        itemRewarded: BASIC_MATERIAL.WOOL_FABRIC,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.HARDENED_LEATHER]: {
        id: FORGE_RECIPE.HARDENED_LEATHER,
        name: 'Forge Hardened Leather',
        materials: [RAW_MATERIALS.LEATHER, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [5, 30],
        statsRequired: { might: 0, speed: 2, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 1800,
        levelRequired: 10,
        itemRewarded: BASIC_MATERIAL.HARDENED_LEATHER,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.COTTON_FABRIC]: {
        id: FORGE_RECIPE.COTTON_FABRIC,
        name: 'Forge Cotton Fabric',
        materials: [RAW_MATERIALS.COTTON, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [10, 30],
        statsRequired: { might: 0, speed: 2, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 1800,
        levelRequired: 10,
        itemRewarded: BASIC_MATERIAL.COTTON_FABRIC,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.SILK_FABRIC]: {
        id: FORGE_RECIPE.SILK_FABRIC,
        name: 'Forge Silk Fabric',
        materials: [RAW_MATERIALS.SILK, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],

        materialsAmounts: [10, 30],
        statsRequired: { might: 0, speed: 2, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 1800,
        levelRequired: 10,
        itemRewarded: BASIC_MATERIAL.SILK_FABRIC,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.COPPER_BAR]: {
        id: FORGE_RECIPE.COPPER_BAR,
        name: 'Forge a Cooper Bar',
        materials: [
            RAW_MATERIALS.COPPER,
            RAW_MATERIALS.COAL,
            RAW_MATERIALS.GOLD,
        ],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
        ],
        materialsAmounts: [10, 2, 20],
        statsRequired: { might: 2, speed: 5, intellect: 2 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 4500,
        levelRequired: 15,
        itemRewarded: BASIC_MATERIAL.COPPER_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.BRONZE_BAR]: {
        id: FORGE_RECIPE.BRONZE_BAR,
        name: 'Forge a Bronze Bar',
        materials: [
            RAW_MATERIALS.BRONZE,
            RAW_MATERIALS.COAL,
            RAW_MATERIALS.GOLD,
        ],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
        ],
        materialsAmounts: [10, 2, 20],
        statsRequired: { might: 4, speed: 7, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 6300,
        levelRequired: 25,
        itemRewarded: BASIC_MATERIAL.BRONZE_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.IRON_BAR]: {
        id: FORGE_RECIPE.IRON_BAR,
        name: 'Forge an Iron Bar',
        materials: [RAW_MATERIALS.IRON, RAW_MATERIALS.COAL, RAW_MATERIALS.GOLD],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
        ],
        materialsAmounts: [10, 2, 20],
        statsRequired: { might: 6, speed: 10, intellect: 4 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 9000,
        levelRequired: 35,
        itemRewarded: BASIC_MATERIAL.IRON_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.SILVER_BAR]: {
        id: FORGE_RECIPE.SILVER_BAR,
        name: 'Forge a Silver Bar',
        materials: [RAW_MATERIALS.SILVER, RAW_MATERIALS.COAL],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [100, 2],
        statsRequired: { might: 5, speed: 8, intellect: 5 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 7200,
        levelRequired: 25,
        itemRewarded: BASIC_MATERIAL.SILVER_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.GOLD_BAR]: {
        id: FORGE_RECIPE.GOLD_BAR,
        name: 'Forge a Gold Bar',
        materials: [RAW_MATERIALS.GOLD, RAW_MATERIALS.COAL],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [100, 2],
        statsRequired: { might: 5, speed: 8, intellect: 5 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 7200,
        levelRequired: 25,
        itemRewarded: BASIC_MATERIAL.GOLD_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.STEEL_BAR]: {
        id: FORGE_RECIPE.STEEL_BAR,
        name: 'Forge a Steel Bar',
        materials: [
            BASIC_MATERIAL.IRON_BAR,
            RAW_MATERIALS.COAL,
            RAW_MATERIALS.GOLD,
        ],
        materialsTypes: [
            RESOURCE_TYPE.BASIC,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
        ],
        materialsAmounts: [1, 5, 50],
        statsRequired: { might: 8, speed: 14, intellect: 5 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 12600,
        levelRequired: 40,
        itemRewarded: BASIC_MATERIAL.STEEL_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.COBALT_BAR]: {
        id: FORGE_RECIPE.COBALT_BAR,
        name: 'Forge a Cobalt Bar',
        materials: [
            RAW_MATERIALS.COBALT,
            RAW_MATERIALS.COAL,
            RAW_MATERIALS.GOLD,
            BASIC_MATERIAL.STEEL_BAR,
        ],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.BASIC,
        ],
        materialsAmounts: [10, 5, 60, 1],
        statsRequired: { might: 10, speed: 16, intellect: 6 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 14400,
        levelRequired: 45,
        itemRewarded: BASIC_MATERIAL.COBALT_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.PLATINUM_BAR]: {
        id: FORGE_RECIPE.PLATINUM_BAR,
        name: 'Forge a Platinum Bar',
        materials: [
            RAW_MATERIALS.PLATINUM,
            RAW_MATERIALS.COAL,
            RAW_MATERIALS.GOLD,
            BASIC_MATERIAL.STEEL_BAR,
        ],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.BASIC,
        ],

        materialsAmounts: [10, 5, 60, 1],
        statsRequired: { might: 12, speed: 18, intellect: 7 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 16200,
        levelRequired: 50,
        itemRewarded: BASIC_MATERIAL.PLATINUM_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
    [FORGE_RECIPE.ADAMANTINE_BAR]: {
        id: FORGE_RECIPE.ADAMANTINE_BAR,
        name: 'Forge an Adamantine Bar',
        materials: [
            RAW_MATERIALS.ADAMANTINE,
            RAW_MATERIALS.COAL,
            RAW_MATERIALS.GOLD,
            BASIC_MATERIAL.STEEL_BAR,
        ],
        materialsTypes: [
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.RAW,
            RESOURCE_TYPE.BASIC,
        ],
        materialsAmounts: [10, 5, 60, 1],
        statsRequired: { might: 14, speed: 20, intellect: 8 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 18000,
        levelRequired: 55,
        itemRewarded: BASIC_MATERIAL.ADAMANTINE_BAR,
        itemRewardedType: RESOURCE_TYPE.BASIC,
        itemRewardedAmount: 1,
    },
}

export const CRAFT_RECIPES_DATA: {
    [k in CRAFT_RECIPE]: Recipe
} = {
    [CRAFT_RECIPE.BONE_DAGGER]: {
        id: CRAFT_RECIPE.BONE_DAGGER,
        name: 'Craft a Bone Dagger',
        materials: [RAW_MATERIALS.BONES, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [5, 50],
        statsRequired: { might: 1, speed: 1, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 3600,
        levelRequired: 0,
        itemRewarded: ITEM.BONE_DAGGER,
        itemRewardedType: RESOURCE_TYPE.ITEM,
        itemRewardedAmount: 1,
    },
    [CRAFT_RECIPE.BONE_HAMMER]: {
        id: CRAFT_RECIPE.BONE_HAMMER,
        name: 'Craft a Bone Hammer',
        materials: [RAW_MATERIALS.BONES, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [15, 50],
        statsRequired: { might: 2, speed: 2, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 7200,
        levelRequired: 5,
        itemRewarded: ITEM.BONE_HAMMER,
        itemRewardedType: RESOURCE_TYPE.ITEM,
        itemRewardedAmount: 1,
    },
    [CRAFT_RECIPE.BONE_AXE]: {
        id: CRAFT_RECIPE.BONE_AXE,
        name: 'Craft a Bone Axe',
        materials: [RAW_MATERIALS.BONES, RAW_MATERIALS.GOLD],
        materialsTypes: [RESOURCE_TYPE.RAW, RESOURCE_TYPE.RAW],
        materialsAmounts: [15, 50],
        statsRequired: { might: 2, speed: 2, intellect: 3 },
        statsSacrificed: EmptyBaseStats,
        cooldown: 7200,
        levelRequired: 5,
        itemRewarded: ITEM.BONE_AXE,
        itemRewardedType: RESOURCE_TYPE.ITEM,
        itemRewardedAmount: 1,
    },
}
