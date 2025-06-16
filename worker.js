importScripts('./pkg/wasm_in_web_worker.js');

const {FibonacciEval} = wasm_bindgen;

async function init_wasm_in_worker() {

    // Load the Wasm file by awaiting the Promise returned by `wasm_bindgen`.
    await wasm_bindgen('./pkg/wasm_in_web_worker_bg.wasm');

    var fibonacci_eval = FibonacciEval.new();

    while (true) {
        let worker_result = fibonacci_eval.advance()

        self.postMessage(worker_result);
        await new Promise(resolve => setTimeout(resolve, 3000));
    }

};

init_wasm_in_worker();