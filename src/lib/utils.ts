export function padString(hex: string, minlen: number) {
    while (hex.length < minlen) hex = `0${hex}`
    return hex
}

export function arrayToInt(array: ArrayBuffer, size: 1 | 2 | 4 | 8, signed: boolean): bigint {
    let view = new DataView(array);

    switch (size) {
        case 1:
            return BigInt(view.getUint8(0))
        case 2:
            return BigInt(view.getUint16(0, true))
        case 4:
            return BigInt(view.getUint32(0, true))
        case 8:
            return view.getBigUint64(0, true)
    }
}

export function arrayToFloat(array: ArrayBuffer, size: 4 | 8): number {
    let view = new DataView(array);

    switch (size) {
        case 4:
            return view.getFloat32(0, true)
        case 8:
            return view.getFloat64(0, true)
    }
}
