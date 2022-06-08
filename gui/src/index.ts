const wasm = import("../pkg");

wasm.then((m) => m.greet("World!")).catch(console.error);
