import initWasm, * as wasm from "../src/graph_to_rholang_parser.js";
import wasmUrl from "../src/graph_to_rholang_parser_bg.wasm?url";

let _ready = null;

export async function init() {
    if (!_ready) _ready = initWasm(wasmUrl);
    return _ready;
}

export function parse_to_ast(input) {
    if (!_ready) throw new Error("graphl-parser not initialized. Call await init() first.");
    return wasm.parse_to_ast(input);
}
