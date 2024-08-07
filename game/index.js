import initWasm, { Game } from "./lights_out.js";

await initWasm();

const game = new Game();
const gameWorker = new Worker("./game.js");
gameWorker.postMessage({ topic: "gameReady", value: null });
gameWorker.onmessage = (e) => {

	if (!e.data.topic) return;

	switch (e.data.topic) {
		case "tick": tickEvent(e); break;
	}

}

function tickEvent(e) {

	console.log("Tick", new Date().toLocaleString())
	game.tick()

}
