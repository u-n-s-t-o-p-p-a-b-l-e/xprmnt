#!/usr/bin/env node

function countdown (seconds: number): void {
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

function main(): void {
	const args = process.argv.slice(2);
	if (args.length != 1) {
		console.log("Usage: countdown <seconds>");
		return;
	}

}
