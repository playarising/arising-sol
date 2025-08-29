use anchor_lang::prelude::*;

static LEVELS: [Level; 150] = [
    Level { min: 1000, max: 2020 },
    Level { min: 2020, max: 3060 },
    Level { min: 3060, max: 4121 },
    Level { min: 4121, max: 5203 },
    Level { min: 5203, max: 6320 },
    Level { min: 6320, max: 7473 },
    Level { min: 7473, max: 8663 },
    Level { min: 8663, max: 9891 },
    Level { min: 9891, max: 11158 },
    Level { min: 11158, max: 12466 },
    Level { min: 12466, max: 13816 },
    Level { min: 13816, max: 15209 },
    Level { min: 15209, max: 16647 },
    Level { min: 16647, max: 18135 },
    Level { min: 18135, max: 19671 },
    Level { min: 19671, max: 21256 },
    Level { min: 21256, max: 22892 },
    Level { min: 22892, max: 24580 },
    Level { min: 24580, max: 26322 },
    Level { min: 26322, max: 28099 },
    Level { min: 28099, max: 29912 },
    Level { min: 29912, max: 31761 },
    Level { min: 31761, max: 33647 },
    Level { min: 33647, max: 35571 },
    Level { min: 35571, max: 37533 },
    Level { min: 37533, max: 39534 },
    Level { min: 39534, max: 41575 },
    Level { min: 41575, max: 43657 },
    Level { min: 43657, max: 45781 },
    Level { min: 45781, max: 47947 },
    Level { min: 47947, max: 50156 },
    Level { min: 50156, max: 52409 },
    Level { min: 52409, max: 54707 },
    Level { min: 54707, max: 57085 },
    Level { min: 57085, max: 59546 },
    Level { min: 59546, max: 62093 },
    Level { min: 62093, max: 64729 },
    Level { min: 64729, max: 67457 },
    Level { min: 67457, max: 70280 },
    Level { min: 70280, max: 73193 },
    Level { min: 73193, max: 76199 },
    Level { min: 76199, max: 79301 },
    Level { min: 79301, max: 82502 },
    Level { min: 82502, max: 85805 },
    Level { min: 85805, max: 89174 },
    Level { min: 89174, max: 92610 },
    Level { min: 92610, max: 96115 },
    Level { min: 96115, max: 99690 },
    Level { min: 99690, max: 103337 },
    Level { min: 103337, max: 107101 },
    Level { min: 107101, max: 110985 },
    Level { min: 110985, max: 114993 },
    Level { min: 114993, max: 119129 },
    Level { min: 119129, max: 123397 },
    Level { min: 123397, max: 127750 },
    Level { min: 127750, max: 132190 },
    Level { min: 132190, max: 136719 },
    Level { min: 136719, max: 141339 },
    Level { min: 141339, max: 146051 },
    Level { min: 146051, max: 150914 },
    Level { min: 150914, max: 155933 },
    Level { min: 155933, max: 161113 },
    Level { min: 161113, max: 166459 },
    Level { min: 166459, max: 171976 },
    Level { min: 171976, max: 177670 },
    Level { min: 177670, max: 183546 },
    Level { min: 183546, max: 189610 },
    Level { min: 189610, max: 195868 },
    Level { min: 195868, max: 202326 },
    Level { min: 202326, max: 209010 },
    Level { min: 209010, max: 215928 },
    Level { min: 215928, max: 223088 },
    Level { min: 223088, max: 230499 },
    Level { min: 230499, max: 238169 },
    Level { min: 238169, max: 246107 },
    Level { min: 246107, max: 254323 },
    Level { min: 254323, max: 262827 },
    Level { min: 262827, max: 271629 },
    Level { min: 271629, max: 280739 },
    Level { min: 280739, max: 290141 },
    Level { min: 290141, max: 299844 },
    Level { min: 299844, max: 309857 },
    Level { min: 309857, max: 320190 },
    Level { min: 320190, max: 330854 },
    Level { min: 330854, max: 341731 },
    Level { min: 341731, max: 352826 },
    Level { min: 352826, max: 364143 },
    Level { min: 364143, max: 375686 },
    Level { min: 375686, max: 387460 },
    Level { min: 387460, max: 399611 },
    Level { min: 399611, max: 412151 },
    Level { min: 412151, max: 425092 },
    Level { min: 425092, max: 438447 },
    Level { min: 438447, max: 452229 },
    Level { min: 452229, max: 466452 },
    Level { min: 466452, max: 481130 },
    Level { min: 481130, max: 496278 },
    Level { min: 496278, max: 511911 },
    Level { min: 511911, max: 528044 },
    Level { min: 528044, max: 544500 },
    Level { min: 544500, max: 561285 },
    Level { min: 561285, max: 578406 },
    Level { min: 578406, max: 595869 },
    Level { min: 595869, max: 613681 },
    Level { min: 613681, max: 631849 },
    Level { min: 631849, max: 650380 },
    Level { min: 650380, max: 669282 },
    Level { min: 669282, max: 688562 },
    Level { min: 688562, max: 708228 },
    Level { min: 708228, max: 728582 },
    Level { min: 728582, max: 749648 },
    Level { min: 749648, max: 771451 },
    Level { min: 771451, max: 794017 },
    Level { min: 794017, max: 817373 },
    Level { min: 817373, max: 841546 },
    Level { min: 841546, max: 866565 },
    Level { min: 866565, max: 892460 },
    Level { min: 892460, max: 919261 },
    Level { min: 919261, max: 947000 },
    Level { min: 947000, max: 975627 },
    Level { min: 975627, max: 1005170 },
    Level { min: 1005170, max: 1035658 },
    Level { min: 1035658, max: 1067122 },
    Level { min: 1067122, max: 1099593 },
    Level { min: 1099593, max: 1132713 },
    Level { min: 1132713, max: 1166495 },
    Level { min: 1166495, max: 1200953 },
    Level { min: 1200953, max: 1236100 },
    Level { min: 1236100, max: 1271950 },
    Level { min: 1271950, max: 1308517 },
    Level { min: 1308517, max: 1345815 },
    Level { min: 1345815, max: 1383859 },
    Level { min: 1383859, max: 1422664 },
    Level { min: 1422664, max: 1462245 },
    Level { min: 1462245, max: 1503093 },
    Level { min: 1503093, max: 1545248 },
    Level { min: 1545248, max: 1588752 },
    Level { min: 1588752, max: 1633648 },
    Level { min: 1633648, max: 1680115 },
    Level { min: 1680115, max: 1728208 },
    Level { min: 1728208, max: 1777984 },
    Level { min: 1777984, max: 1829502 },
    Level { min: 1829502, max: 1882823 },
    Level { min: 1882823, max: 1938010 },
    Level { min: 1938010, max: 1995129 },
    Level { min: 1995129, max: 2054247 },
    Level { min: 2054247, max: 2115434 },
    Level { min: 2115434, max: 2178763 },
    Level { min: 2178763, max: 2244309 },
    Level { min: 2244309, max: 5000000 },
];

#[inline(always)]
pub fn get_level(experience: u64) -> u16 {
    let mut n: u16 = 0;

    loop {
        if experience >= LEVELS[n as usize].min && experience < LEVELS[n as usize].max {
            return n;
        }
        if n > 150 {
            return 150;
        }
        n += 1;
    }
}

struct Level {
    min: u64,
    max: u64,
}

/// The size of the character stats.
pub const BASE_STATS_SIZE: usize =
    16 + // might
    16 + // speed
    16; // intellect

/// The struct for character stats.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BaseStats {
    pub might: u16,
    pub speed: u16,
    pub intellect: u16,
}

/// The size of the character attributes.
pub const BASE_ATTRIBUTES_SIZE: usize =
    16 + // atk
    16 + // def
    16 + // range
    16 + // mag_atk
    16 + // mag_def
    16; // rate

/// The struct for character attributes.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BaseAttributes {
    pub atk: u16,
    pub def: u16,
    pub range: u16,
    pub mag_atk: u16,
    pub mag_def: u16,
    pub rate: u16,
}

pub enum ResourceType {
    Raw = 1,
    Basic,
}

/// The size of a craft and forge recipe.
pub const RECIPE_SIZE: usize =
    8 + // discriminator
    32 + // id
    24 + // name
    320 + // materials
    320 + // materials_amounts
    150 + // materials_types
    BASE_STATS_SIZE + // stats_required
    BASE_STATS_SIZE + // stats_sacrificed
    32 + // cooldown
    16 + // level_required
    32 + // item_rewarded
    32 + // item_rewarded_amount
    16 + // item_rewarded_type
    1; // available

/// The full metadata information for a recipe.
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub materials: [u32; 10],
    pub materials_amounts: [u32; 10],
    pub materials_types: [u16; 10],
    pub stats_required: BaseStats,
    pub stats_sacrificed: BaseStats,
    pub cooldown: u32,
    pub level_required: u16,
    pub item_rewarded: u32,
    pub item_rewarded_amount: u32,
    pub item_rewarded_type: u16,
    pub available: bool,
}