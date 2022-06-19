const wasm = import('../gui/pkg/index');

wasm.then((m) => {
    m.start()
}).catch(console.error);
