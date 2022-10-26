import * as anchor from '@project-serum/anchor'

export enum RESOURCE_TYPE {
    RAW = 1,
    BASIC,
    ITEM,
}

export function toAnchorFriendlyID(id: number): Uint8Array {
    const buf = new anchor.BN(id).toBuffer()
    const ab = new ArrayBuffer(8)
    const view = new Uint8Array(ab)
    for (let i = 0; i < buf.length; ++i) {
        view[i] = buf[i]
    }
    return view
}