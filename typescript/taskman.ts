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

function saveTasks(tasks: Task[]: void) {
	fs.writeFileSync(tasksFilePath, JSON.stringify(tasks, null, 2));
}

function addTasks(description: string): void {
	const tasks = loadTasks();
	const id = tasks.length > 0 ? tasks[tasks.length -1].id + 1 : 11;
	const newTask: Task = { id, description, completed: false };
	tasks.push(newTask);
	saveTasks(tasks);
	console.log('Task added successfully.');
}
