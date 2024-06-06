#!/usr/bin/env node
function countdown(seconds) {
    var remainingSeconds = seconds;
    var interval = setInterval(function () {
        if (remainingSeconds <= 0) {
            clearInterval(interval);
            console.log("Countdown complete!");
        }
        else {
            console.log("".concat(remainingSeconds, " seconds remaining..."));
            remainingSeconds--;
        }
    }, 1000);
}
function main() {
    var args = process.argv.slice(2);
    if (args.length != 1) {
        console.log("Usage: countdown <seconds>");
        return;
    }
    var seconds = parseInt(args[0]);
    if (isNaN(seconds) || seconds <= 0) {
        console.log("Invalid input. Please provide a positive integer for the number of seconds.");
        return;
    }
    countdown(seconds);
}
main();
