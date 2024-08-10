import initWasm, { Game } from "./lights_out.js";

await initWasm();

// Settings

let PRINT_TICK = false;

const $loading = document.getElementById("loading");
const $loadingDescription = $loading.querySelector(":scope .loading-description");

// Game

$loading.classList.add("active");
$loadingDescription.textContent = "Initializing game";

const game = new Game();
const gameWorker = new Worker("./game.js");
gameWorker.postMessage({ topic: "gameReady", value: null });
gameWorker.onmessage = (e) => {

	if (!e.data.topic) return;

	switch (e.data.topic) {
		case "tick": tickEvent(e); break;
	}

}

// Things

$loadingDescription.textContent = "Loading things";
game.load_things();

// Start Game

$loading.classList.remove("active");
gameWorker.postMessage({ topic: "startTicker", value: null });

// Event Handlers

function tickEvent(e) {

	game.tick()

	if (PRINT_TICK)	console.debug("Tick", e.data.value, new Date().toLocaleString())

}

// Export

window.game = game;
