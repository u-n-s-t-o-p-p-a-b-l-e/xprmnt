#!/usr/bin/env node

function countdown (seconds: number) {
	let remainingSeconds = seconds;
	const interval = setInterval(() => {
		if (remainingSeconds <= 0) {
			clearInterval(interval);
			console.log("Countdown complete!");
		} else {
			console.log(`${remainingSeconds} seconds remaining...`);
			remainingSeconds--;
		}

	}, 1000);
}
