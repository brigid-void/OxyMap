// Initialize and load Wasm module
import init, * as wasm from './pkg/oxy_wasm.js';

async function run() {
    await init();
    // Expose Wasm functions to window
    window.wasm = wasm;
}

run().catch(console.error);
