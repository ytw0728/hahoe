const wasm = import('../dist-wasm');

wasm.then((m) => {
    m.main()
}).catch(console.error);
