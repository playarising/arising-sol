export enum BASIC_MATERIAL {
    WOOD_PLANK = 1,
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

interface BasicMaterialData {
    id: BASIC_MATERIAL
    name: string
}

export const BASIC_MATERIALS_DATA: {
    [k in BASIC_MATERIAL]: BasicMaterialData
} = {
    [BASIC_MATERIAL.WOOD_PLANK]: {
        id: BASIC_MATERIAL.WOOD_PLANK,
        name: 'Wood Plank',
    },
    [BASIC_MATERIAL.IRONSTONE]: {
        id: BASIC_MATERIAL.IRONSTONE,
        name: 'Ironstone',
    },
    [BASIC_MATERIAL.WOOL_FABRIC]: {
        id: BASIC_MATERIAL.WOOL_FABRIC,
        name: 'Wool Fabric',
    },
    [BASIC_MATERIAL.HARDENED_LEATHER]: {
        id: BASIC_MATERIAL.HARDENED_LEATHER,
        name: 'Hardened Leather',
    },
    [BASIC_MATERIAL.COTTON_FABRIC]: {
        id: BASIC_MATERIAL.COTTON_FABRIC,
        name: 'Cotton Fabric',
    },
    [BASIC_MATERIAL.SILK_FABRIC]: {
        id: BASIC_MATERIAL.SILK_FABRIC,
        name: 'Silk Fabric',
    },
    [BASIC_MATERIAL.COPPER_BAR]: {
        id: BASIC_MATERIAL.COPPER_BAR,
        name: 'Cooper Bar',
    },
    [BASIC_MATERIAL.BRONZE_BAR]: {
        id: BASIC_MATERIAL.BRONZE_BAR,
        name: 'Bronze Bar',
    },
    [BASIC_MATERIAL.IRON_BAR]: {
        id: BASIC_MATERIAL.IRON_BAR,
        name: 'Iron Bar',
    },
    [BASIC_MATERIAL.SILVER_BAR]: {
        id: BASIC_MATERIAL.SILVER_BAR,
        name: 'Silver Bar',
    },
    [BASIC_MATERIAL.GOLD_BAR]: {
        id: BASIC_MATERIAL.GOLD_BAR,
        name: 'Gold Bar',
    },
    [BASIC_MATERIAL.STEEL_BAR]: {
        id: BASIC_MATERIAL.STEEL_BAR,
        name: 'Steel Bar',
    },
    [BASIC_MATERIAL.COBALT_BAR]: {
        id: BASIC_MATERIAL.COBALT_BAR,
        name: 'Cobalt Bar',
    },
    [BASIC_MATERIAL.PLATINUM_BAR]: {
        id: BASIC_MATERIAL.PLATINUM_BAR,
        name: 'Platinum Bar',
    },
    [BASIC_MATERIAL.ADAMANTINE_BAR]: {
        id: BASIC_MATERIAL.ADAMANTINE_BAR,
        name: 'Adamantine Bar',
    },
}
