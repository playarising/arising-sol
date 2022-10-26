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

export function toNormalBaseStats(stats: AnchorFriendlyBaseStats): BaseStats {
    return {
        might: stats.might.toNumber(),
        speed: stats.speed.toNumber(),
        intellect: stats.intellect.toNumber(),
    }
}

export interface AnchorFriendlyBaseAttributes {
    atk: anchor.BN
    def: anchor.BN
    range: anchor.BN
    magAtk: anchor.BN
    magDef: anchor.BN
    rate: anchor.BN
}

export interface BaseAttributes {
    atk: number
    def: number
    range: number
    magAtk: number
    magDef: number
    rate: number
}

export function toAnchorFriendlyBaseAttributes(
    attributes: BaseAttributes
): AnchorFriendlyBaseAttributes {
    return {
        atk: new anchor.BN(attributes.atk),
        def: new anchor.BN(attributes.def),
        range: new anchor.BN(attributes.range),
        magAtk: new anchor.BN(attributes.magAtk),
        magDef: new anchor.BN(attributes.magDef),
        rate: new anchor.BN(attributes.rate),
    }
}

export function toNormalBaseAttributes(
    attributes: AnchorFriendlyBaseAttributes
): BaseAttributes {
    return {
        atk: attributes.atk.toNumber(),
        def: attributes.atk.toNumber(),
        range: attributes.range.toNumber(),
        magAtk: attributes.magAtk.toNumber(),
        magDef: attributes.magDef.toNumber(),
        rate: attributes.rate.toNumber(),
    }
}

export const EmptyBaseStats: BaseStats = { might: 0, speed: 0, intellect: 0 }

export const EmptyAttributes: BaseAttributes = {
    atk: 0,
    def: 0,
    range: 0,
    magAtk: 0,
    magDef: 0,
    rate: 0,
}
