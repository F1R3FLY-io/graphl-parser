import initWasm, * as wasm from "../src/graph_to_rholang_parser.js";

let _ready = null;

async function init() {
    if (!_ready) {
        import wasmUrl from "../src/graph_to_rholang_parser_bg.wasm?url";
        _ready = initWasm(wasmUrl);
    }
    return _ready;
}

export { init };
export default init;

export function parse_to_ast(input) {
    if (!_ready) throw new Error("graphl-parser not initialized. Call await init() first.");
    return wasm.parse_to_ast(input);
}
