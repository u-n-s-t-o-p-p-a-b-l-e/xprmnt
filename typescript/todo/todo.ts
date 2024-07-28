#!/usr/bin/env node 

import * as readlineSync from 'readline-sync';
import * as fs from 'fs';

interface TodoItem {
	id: number;
	task: string;
	completed: boolean;
}

let todos: TodoItem[] = [];

function loadTodos(): void {
	try {
		const data = fs.readFileSync('todos.json', 'utf-8');
		todos = JSON.parse(data);
	} catch (error) {
		console.log("No todos found. Starting with an empty list.");
	}
}

function saveTodos(): void {
	fs.writeFileSync('todos.json', JSON.stringify(todos, null, 2));
}

function displayMenu(): void {
	console.log("Todo List Manager");
	console.log("1. View Todos");
	console.log("2. Add todo");
	console.log("3. Mark Todo as Completed");
	console.log("4. Exit");
}

function viewTodos(): void {
	console.log("Todos:");
	todos.forEach( todo => {
		console.log(`${todo.id}. [${todo.completed ? 'X' : ' '}] ${todo.task}`);
	});
}

function addTodo(): void {
	const task = readlineSync.question("Enter the task: ");
	const id = todos.length + 1;
	const newTodo: TodoItem = { id, task, completed: false };
	todos.push(newTodo);
	saveTodos();
	console.log("Todo added successfully");
}

function markTodoAsCompleted(): void {
	const id = parseInt(readlineSync.question("Enter the ID of the todo to mark as completed"));
	const todo = todos.find(todo => todo.id === id);
	if (todo) {
		todo.completed = true;
		saveTodos();
		console.log("Todo marked as completed.");
	} else {
		console.log("Todo not found");
	}
}

function main(): void {
	loadTodos();

	let choice = 0;

	while (choice !== 4) {
		displayMenu();
		choice = parseInt(readlineSync.question("Enter your choice: "));

		switch (choice) {
			case 1:
				viewTodos();
			break;
			case 2:
				addTodo();
			break;
			case 3:
				markTodoAsCompleted();
			break;
			case 4:
				console.log("Exiting. Goodbye!");
			break;
			default:
				console.log("Invalid choice. Please choose again");
		}
	}
}

main();
