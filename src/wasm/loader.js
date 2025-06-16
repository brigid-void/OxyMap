import init, { memory } from '/pkg/oxy_wasm.js';

/**
 * WasmBridge class manages the WebAssembly module lifecycle, including
 * loading, data handling, and memory monitoring.
 */
class WasmBridge {
    constructor() {
        this.wasm = null;
        this.memory = null;
    }

    /**
     * Initializes the WebAssembly module.
     * It loads the .wasm file, sets up the necessary exports,
     * and starts monitoring memory usage.
     */
    async init() {
        try {
            // The wasm-bindgen `init` function takes an optional URL to the .wasm file.
            // We pass it here to load from '/pkg/oxy_wasm_bg.wasm' as requested.
            const wasmModule = await init('/pkg/oxy_wasm_bg.wasm');
            this.wasm = wasmModule;
            this.memory = memory; // `memory` is an export from the wasm-bindgen JS file.

            // Expose core methods from the Wasm module.
            // Assumes 'load_data', 'apply_filters', and 'export_csv' are exported from Rust.
            this.loadData = this.wasm.load_data;
            this.applyFilters = this.wasm.apply_filters;
            this.exportCSV = this.wasm.export_csv;

            console.log("WasmBridge initialized successfully.");

            // Start monitoring Wasm memory usage.
            this.memoryMonitor();

        } catch (error) {
            console.error("Failed to compile or instantiate Wasm module:", error);
            // Provide a user-friendly message for compilation failures.
            alert("Error: Failed to load core application components. Please see the console for details.");
            throw error;
        }
    }

    /**
     * Fetches a dataset from the server and loads it into Wasm memory.
     * @param {string} name - The name of the dataset to load (e.g., 'points').
     */
    async loadDataset(name) {
        try {
            const response = await fetch(`/data/${name}.fgb`);
            if (!response.ok) {
                throw new Error(`Failed to fetch dataset: ${response.statusText}`);
            }
            const arrayBuffer = await response.arrayBuffer();

            // The wasm-bindgen generated 'load_data' function receives the data.
            // It's responsible for managing memory, including growth if necessary.
            this.loadData(new Uint8Array(arrayBuffer));
            console.log(`Dataset '${name}' loaded into Wasm.`);

        } catch (error) {
            console.error(`Error loading dataset '${name}':`, error);
            throw error;
        }
    }

    /**
     * Periodically logs the current Wasm memory usage to the console.
     */
    memoryMonitor() {
        // Set up an interval to log memory usage every 30 seconds.
        setInterval(() => {
            if (this.memory) {
                const memoryUsage = this.memory.buffer.byteLength / (1024 * 1024); // in MB
                console.log(`Wasm memory usage: ${memoryUsage.toFixed(2)} MB`);
            }
        }, 30000);
    }
}

// Export the WasmBridge class as the default export of this ES6 module.
export default WasmBridge;
