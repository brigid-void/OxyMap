// JS-Wasm binding functions
export function getGeojsonData(buffer) {
    return wasm.parse_flatgeobuf(new Uint8Array(buffer));
}
