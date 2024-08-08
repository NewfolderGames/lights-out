let ticks = 0;

self.onmessage = e => {

	if (!e.data.topic) return;

	switch (e.data.topic) {
		case "gameReady": gameReadyEvent(e); break;
		case "startTicker": startTickerEvent(e); break;
	}

};

function gameReadyEvent(e) {

	self.postMessage({ topic: "workerReady", value: null });

}

function startTickerEvent(e) {

	setInterval(() => {

		self.postMessage({ topic: "tick", value: ++ticks });

	}, 1000);

}
