#!/ust/bin/env node

import * as readlineSync from 'readline-sync';

function add(a: number, b: number): number {
	return a + b;
}

function substract(a: number, b: number): number {
	return a - b;
}

function multiply(a: number, b: number): number {
	return a * b;
}
function divide(a: number, b: number): number {
	if (b === 0) {
		throw new Error("Division by zero is not allowed");
	}
	return a / b;
}

function displayMenu (): void {
	console.log("1. Add");
	console.log("2. Substract");
	console.log("3. Multiply");
	console.log("4. Divide");
	console.log("5. Exit");
}

function getUserInput (prompt: string): number {
	return parseFloat(readlineSync.question(prompt));
}

function main(): void {
	let choice = 0;

	while (choice !== 5) {
		displayMenu();
		choice = getUserInput("Enter your choice: ");

		if (choice >= 1 && choice <= 4) {
			const num1 = getUserInput("Enter first number: ");
			const num2 = getUserInput("Enter second number: ");
			let result: number;

			switch (choice) {
				case 1:
					result = add(num1, num2);
					break;
				case 2:
					result = substract(num1, num2);
					break;
				case 3:
					result = multiply(num1, num2);
					break;
				case 4:
					result = divide(num1, num2);
					break;
				default:
					throw new Error("Invalid choice");
					
			}
			console.log("Result:", result);
		} else if (choice !== 5) {
			console.log("Invalid choice. Please choose again.");
		}
	}
}

main();
