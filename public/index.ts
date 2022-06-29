const wasm = import('../gui/pkg/gui');

wasm.then((m) => {
    m.start()
}).catch(console.error);
