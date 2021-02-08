worker = self;
import("bpxe/bpxe_bg.wasm")
.then(wasm => {
	import("bpxe/bpxe.js").then(module => {
		postMessage("started");
		worker.onmessage = (msg) => {
			let channel = module.Channel.from(msg.data);
			while (true) {
				channel.run();
				console.debug("worker: runner terminated, restarting");
			}
		}
	})
});
