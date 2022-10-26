export enum RAW_MATERIALS {
    WOOD = 1,
    BONES,
    COPPER,
    BRONZE,
    STONE,
    IRON,
    LEATHER,
    COTTON,
    WOOL,
    SILK,
    SILVER,
    GOLD,
    COAL,
    COBALT,
    PLATINUM,
    ADAMANTINE,
}

interface RawMaterialsData {
    id: RAW_MATERIALS
    name: string
}

export const RAW_MATERIALS_DATA: { [k in RAW_MATERIALS]: RawMaterialsData } = {
    [RAW_MATERIALS.WOOD]: { id: RAW_MATERIALS.WOOD, name: 'Wood' },
    [RAW_MATERIALS.BONES]: { id: RAW_MATERIALS.BONES, name: 'Bones' },
    [RAW_MATERIALS.COPPER]: { id: RAW_MATERIALS.COPPER, name: 'Cooper' },
    [RAW_MATERIALS.BRONZE]: { id: RAW_MATERIALS.BRONZE, name: 'Bronze' },
    [RAW_MATERIALS.STONE]: { id: RAW_MATERIALS.STONE, name: 'Stone' },
    [RAW_MATERIALS.IRON]: { id: RAW_MATERIALS.IRON, name: 'Iron' },
    [RAW_MATERIALS.LEATHER]: { id: RAW_MATERIALS.LEATHER, name: 'Leather' },
    [RAW_MATERIALS.COTTON]: { id: RAW_MATERIALS.COTTON, name: 'Cotton' },
    [RAW_MATERIALS.WOOL]: { id: RAW_MATERIALS.WOOL, name: 'Wool' },
    [RAW_MATERIALS.SILK]: { id: RAW_MATERIALS.SILK, name: 'Silk' },
    [RAW_MATERIALS.SILVER]: { id: RAW_MATERIALS.SILVER, name: 'Silver' },
    [RAW_MATERIALS.GOLD]: { id: RAW_MATERIALS.GOLD, name: 'Gold' },
    [RAW_MATERIALS.COAL]: { id: RAW_MATERIALS.COAL, name: 'Coal' },
    [RAW_MATERIALS.COBALT]: { id: RAW_MATERIALS.COBALT, name: 'Cobalt' },
    [RAW_MATERIALS.PLATINUM]: { id: RAW_MATERIALS.PLATINUM, name: 'Platinum' },
    [RAW_MATERIALS.ADAMANTINE]: {
        id: RAW_MATERIALS.ADAMANTINE,
        name: 'Adamantine',
    },
}
