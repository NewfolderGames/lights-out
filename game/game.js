let game;
let wasm;

self.onmessage = e => {

	if (!e.data.topic) return;

	switch (e.data.topic) {
		case "gameReady": gameReadyEvent(e); break;
	}

};

async function gameReadyEvent(e) {

	console.log("Game Ready!");
	console.log("Starting ticker.");

	setInterval(() => {

		self.postMessage({ topic: "tick", value: null });

	}, 1000);

}
