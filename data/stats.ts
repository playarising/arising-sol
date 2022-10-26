import * as anchor from '@project-serum/anchor'

export interface AnchorFriendlyBaseStats {
    might: anchor.BN
    speed: anchor.BN
    intellect: anchor.BN
}

export interface BaseStats {
    might: number
    speed: number
    intellect: number
}

export function toAnchorFriendlyBaseStats(
    stats: BaseStats
): AnchorFriendlyBaseStats {
    return {
        might: new anchor.BN(stats.might),
        speed: new anchor.BN(stats.speed),
        intellect: new anchor.BN(stats.intellect),
    }
}

export interface AnchorFriendlyBaseAttributes {
    atk: anchor.BN
    def: anchor.BN
    range: anchor.BN
    mag_atk: anchor.BN
    mag_def: anchor.BN
    rate: anchor.BN
}

export interface BaseAttributes {
    atk: number
    def: number
    range: number
    mag_atk: number
    mag_def: number
    rate: number
}

export function toAnchorFriendlyBaseAttributes(
    attributes: BaseAttributes
): AnchorFriendlyBaseAttributes {
    return {
        atk: new anchor.BN(attributes.atk),
        def: new anchor.BN(attributes.def),
        range: new anchor.BN(attributes.range),
        mag_atk: new anchor.BN(attributes.mag_atk),
        mag_def: new anchor.BN(attributes.mag_def),
        rate: new anchor.BN(attributes.rate),
    }
}

export const EmptyBaseStats: BaseStats = { might: 0, speed: 0, intellect: 0 }

export const EmptyAttributes: BaseAttributes = {
    atk: 0,
    def: 0,
    range: 0,
    mag_atk: 0,
    mag_def: 0,
    rate: 0,
}
