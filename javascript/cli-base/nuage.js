#!/usr/bin/env node 

const readline = require('readline');

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout
});

const askQuestion = (question) => {
	return new Promise((resolve) => rl.question(question, resolve));
};

const main = async () => {
	try {
		const name = await askQuestion('What is your name? ');
		const age = await askQuestion('How old are you? ');

		const ageNum = parseInt(age, 10);

		if (isNaN(ageNum)) {
			console.log('Please enter a valid number for age.');
		} else {
			if (ageNum >= 18) {
				console.log(`Hello, ${name}. You are an adult.`)
			} else {
				console.log(`Hello, ${name}. You are not an adult`);
			}
		}

	} catch (error) {
		console.error('An error occured: ', error);
	} finally {
		rl.close();
	}
};

main();
