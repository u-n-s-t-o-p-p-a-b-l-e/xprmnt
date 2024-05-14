#!/usr/bin//env node 

import * as fs from 'fs';

interface Task {
	id: number;
	description: string;
	completed: boolean;
}

const tasksFilePath = 'tasks.json';

function loadTasks(): Task[] {
	try {
		const data = fs.readFileSync(tasksFilePath, 'utf-8');
		return JSON.parse(data);
	} catch (error) {
		return [];
	}
}

function saveTasks(tasks: Task[]): void {
	fs.writeFileSync(tasksFilePath, JSON.stringify(tasks, null, 2));
}

function addTask(description: string): void {
	const tasks = loadTasks();
	const id = tasks.length > 0 ? tasks[tasks.length -1].id + 1 : 11;
	const newTask: Task = { id, description, completed: false };
	tasks.push(newTask);
	saveTasks(tasks);
	console.log('Task added successfully.');
}

function listTasks(): void {
	const tasks = loadTasks();
	console.log('Your Tasks: ');
	tasks.forEach((task, index) => {
		console.log(`${index +1}. [${task.completed ? '✔' : ' '}] ${task.description}`);
	})
}

function markTaskAsCompleted(id: number): void {
	let tasks = loadTasks();
	const taskIndex = tasks.findIndex(task => task.id === id);
	if (taskIndex !== -1) {
		tasks[taskIndex].completed = true;
		saveTasks(tasks);
		console.log('Task marked as completed');
	} else {
		console.log(' Task not found. ');
	}
}

function deleteTask(id: number): void {
	let tasks = loadTasks();
	const initialLength = tasks.length;
	tasks = tasks.filter(task => task.id !== id);
	if (tasks.length === initialLength) {
		console.log('Task not found');
	} else {
		saveTasks(tasks);
		console.log('Task deleted successfully');
	}
}

function main(): void {
	const [command, ...args] = process.argv.slice(2);

	switch (command) {
		case 'add':
			const description = args.join(' ');
		addTask(description);
		break;
		case 'list':
			listTasks();
		break;
		case 'complete':
			const taskId = parseInt(args[0]);
		if (isNaN(taskId)) {
			console.log('Invalid task ID.');
		} else {
			markTaskAsCompleted(taskId);
		}
		break;
		case 'delete':
			const deleteId = parseInt(args[0]);
		if (isNaN(deleteId)) {
			console.log('Invalid task ID.');
		} else {
			deleteTask(deleteId);
		}
		break;
		default:
			console.log('Invalid command. Usage: tasks add|list|complete <id>|delete <id>');
	}
}

main();
