import * as anchor from '@project-serum/anchor'

export interface BaseStats {
    might: number
    speed: number
    intellect: number
}

export interface BaseAttributes {
    atk: number
    def: number
    range: number
    magAtk: number
    magDef: number
    rate: number
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
