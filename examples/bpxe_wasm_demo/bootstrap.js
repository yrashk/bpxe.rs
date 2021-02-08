import("bpxe/bpxe_bg.wasm")
.then(wasm => import("./index.js"))
.catch(e => console.error("Error importing `index.js`:", e));
