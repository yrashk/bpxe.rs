importScripts("node_modules/bpxe/web/bpxe.js");
wasm_bindgen("node_modules/bpxe/web/bpxe_bg.wasm").then(() => {
	self.module = wasm_bindgen;
	postMessage("started")
});

onmessage = (msg) => {
	let channel = self.module.Channel.from(msg.data);
	while (true) {
	  channel.run(self);
	  console.debug("worker: runner terminated, restarting");
	}
}
