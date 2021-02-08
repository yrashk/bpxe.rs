import * as bpxe from "bpxe";
import demo from "./demo.bpmn";
import Worker from "./worker.js";
window.demo = demo;

let worker = Worker();

worker.onmessage = (e) => {
	if (e.data == "started") {
		let channel = new bpxe.Channel();
		worker.postMessage(channel.replica());
		window.bpxe = channel.sender();
		console.log("Use `bpxe` object to interact with BPXE");
	} else {
		console.log(e.data);
	}
}
