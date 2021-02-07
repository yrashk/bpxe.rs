import * as bpxe from "bpxe";
import demo from "./demo.bpmn";
window.demo = demo;

let worker = new Worker("worker.js");

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
