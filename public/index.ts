const wasm = import('../dist-wasm');

wasm.then((m) => {
    m.start()
}).catch(console.error);
