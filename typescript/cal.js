#!/ust/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var readlineSync = require("readline-sync");
function add(a, b) {
    return a + b;
}
function substract(a, b) {
    return a - b;
}
function multiply(a, b) {
    return a * b;
}
function divide(a, b) {
    if (b === 0) {
        throw new Error("Division by zero is not allowed");
    }
    return a / b;
}
function displayMenu() {
    console.log("1. Add");
    console.log("2. Substract");
    console.log("3. Multiply");
    console.log("4. Divide");
    console.log("5. Exit");
}
function getUserInput(prompt) {
    return parseFloat(readlineSync.question(prompt));
}
function main() {
    var choice = 0;
    while (choice !== 5) {
        displayMenu();
        choice = getUserInput("Enter your choice: ");
        if (choice >= 1 && choice <= 4) {
            var num1 = getUserInput("Enter first number: ");
            var num2 = getUserInput("Enter second number: ");
            var result = void 0;
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
        }
        else if (choice !== 5) {
            console.log("Invalid choice. Please choose again.");
        }
    }
}
main();
