#!/usr/bin/env node 

const readline = require('readline');

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout
});

const askQuestion = (question) => {
	return new Promise((resolve) => rl.question(question, resolve));
};

const main =  async () => {
	try {
		const num1 = await askQuestion('Enter the first number: ');

		const num2 = await askQuestion('Enter the second number: ');

		const sum = parseFloat(num1) + parseFloat(num2);

		console.log(`The sum of ${num1} and ${num2} is ${sum}`);
	} catch (error) {
		console.error('An error occurred: ', error);

	} finally {
		rl.close();
	}
};

main();
