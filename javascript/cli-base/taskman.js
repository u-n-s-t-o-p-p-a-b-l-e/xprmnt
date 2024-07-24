#!/usr/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
var tasksFilePath = 'tasks.json';
function loadTasks() {
    try {
        var data = fs.readFileSync(tasksFilePath, 'utf-8');
        return JSON.parse(data);
    }
    catch (error) {
        return [];
    }
}
function saveTasks(tasks) {
    fs.writeFileSync(tasksFilePath, JSON.stringify(tasks, null, 2));
}
function addTask(description) {
    var tasks = loadTasks();
    var id = tasks.length > 0 ? tasks[tasks.length - 1].id + 1 : 1;
    var newTask = { id: id, description: description, completed: false };
    tasks.push(newTask);
    saveTasks(tasks);
    console.log('Task added successfully.');
}
function listTasks() {
    var tasks = loadTasks();
    console.log('Your Tasks:');
    tasks.forEach(function (task, index) {
        console.log("".concat(index + 1, ". [").concat(task.completed ? '✔' : ' ', "] ").concat(task.description));
    });
}
function markTaskAsCompleted(id) {
    var tasks = loadTasks();
    var taskIndex = tasks.findIndex(function (task) { return task.id === id; });
    if (taskIndex !== -1) {
        tasks[taskIndex].completed = true;
        saveTasks(tasks);
        console.log('Task marked as completed.');
    }
    else {
        console.log('Task not found.');
    }
}
function deleteTask(id) {
    var tasks = loadTasks();
    var initialLength = tasks.length;
    tasks = tasks.filter(function (task) { return task.id !== id; });
    if (tasks.length === initialLength) {
        console.log('Task not found.');
    }
    else {
        saveTasks(tasks);
        console.log('Task deleted successfully.');
    }
}
function main() {
    var _a = process.argv.slice(2), command = _a[0], args = _a.slice(1);
    switch (command) {
        case 'add':
            var description = args.join(' ');
            addTask(description);
            break;
        case 'list':
            listTasks();
            break;
        case 'complete':
            var taskId = parseInt(args[0]);
            if (isNaN(taskId)) {
                console.log('Invalid task ID.');
            }
            else {
                markTaskAsCompleted(taskId);
            }
            break;
        case 'delete':
            var deleteId = parseInt(args[0]);
            if (isNaN(deleteId)) {
                console.log('Invalid task ID.');
            }
            else {
                deleteTask(deleteId);
            }
            break;
        default:
            console.log('Invalid command. Usage: tasks add|list|complete <id>|delete <id>');
    }
}
main();
