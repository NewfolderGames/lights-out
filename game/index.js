import initWasm, { Game } from "./lights_out.js";

await initWasm();

// Settings

let PRINT_TICK = false;

// Game

const game = new Game();
const gameWorker = new Worker("./game.js");
gameWorker.postMessage({ topic: "gameReady", value: null });
gameWorker.onmessage = (e) => {

	if (!e.data.topic) return;

	switch (e.data.topic) {
		case "workerReady": workerReadyEvent(e); break;
		case "tick": tickEvent(e); break;
	}

}

// Loading

const $loading = document.getElementById("loading");
const $loadingDescription = $loading.querySelector(":scope .loading-description");

$loading.classList.add("active");
$loadingDescription.textContent = "Initializing game";

// Event Handlers

function workerReadyEvent(e) {

	console.info("Worker Ready!");
	console.info("Starting ticker.");

	gameWorker.postMessage({ topic: "startTicker", value: null });

	$loading.classList.remove("active");

}

function tickEvent(e) {

	game.tick()

	if (PRINT_TICK)	console.debug("Tick", e.data.value, new Date().toLocaleString())

}

// Export

window.game = game;
