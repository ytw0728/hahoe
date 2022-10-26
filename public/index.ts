const wasm = import('../dist-wasm');

let tool = new Proxy<{ value: string }>({ value: '' }, {
    get: ({ value }) => value,
    set: (target, props, newValue) => {
        if (props !== 'value') return false
        target[props] = newValue
        return true
    }
});

let tool_events = new Proxy<{list: any[]}>({ list: [] }, {
    get: ({ list }) => list,
    set: (target, props, newValue) => {
        if (props !== 'value') return false
        target[props] = newValue
        return true
    }
});
(window as any).appData = Object.freeze({ tool, tool_events })

wasm.then((m) => {
    m.main()
}).catch(console.error);
