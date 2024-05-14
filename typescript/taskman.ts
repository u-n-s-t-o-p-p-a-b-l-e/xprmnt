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
