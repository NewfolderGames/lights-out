let ticks = 0;

self.onmessage = e => {

	if (!e.data.topic) return;

	switch (e.data.topic) {
		case "startTicker": startTickerEvent(e); break;
	}

};

function startTickerEvent(e) {

	setInterval(() => {

		self.postMessage({ topic: "tick", value: ++ticks });

	}, 1000);

}
