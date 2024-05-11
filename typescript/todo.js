#!/usr/bin/env node 
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var readlineSync = require("readline-sync");
var fs = require("fs");
var todos = [];
function loadTodos() {
    try {
        var data = fs.readFileSync('todos.json', 'utf-8');
        todos = JSON.parse(data);
    }
    catch (error) {
        console.log("No todos found. Starting with an empty list.");
    }
}
function saveTodos() {
    fs.writeFileSync('todos.json', JSON.stringify(todos, null, 2));
}
function displayMenu() {
    console.log("Todo List Manager");
    console.log("1. View Todos");
    console.log("2. Add todo");
    console.log("3. Mark Todo as Completed");
    console.log("4. Exit");
}
function viewTodos() {
    console.log("Todos:");
    todos.forEach(function (todo) {
        console.log("".concat(todo.id, ". [").concat(todo.completed ? 'X' : ' ', "] ").concat(todo.task));
    });
}
function addTodo() {
    var task = readlineSync.question("Enter the task: ");
    var id = todos.length + 1;
    var newTodo = { id: id, task: task, completed: false };
    todos.push(newTodo);
    saveTodos();
    console.log("Todo added successfully");
}
function markTodoAsCompleted() {
    var id = parseInt(readlineSync.question("Enter the ID of the todo to mark as completed"));
    var todo = todos.find(function (todo) { return todo.id === id; });
    if (todo) {
        todo.completed = true;
        saveTodos();
        console.log("Todo marked as completed.");
    }
    else {
        console.log("Todo not found");
    }
}
function main() {
    loadTodos();
    var choice = 0;
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
