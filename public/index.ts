const wasm = import('../gui/pkg');

wasm.then((m) => m.start()).catch(console.error);
