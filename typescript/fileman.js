#!/use/bin/env node 
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
function copyFile(source, destination) {
    fs.copyFileSync(source, destination);
    console.log("File copied from ".concat(source, " to ").concat(destination));
}
function moveFile(source, destination) {
    fs.renameSync(source, destination);
    console.log("File moved from ".concat(source, " to #{destination}"));
}
function deleteFile(filePath) {
    fs.unlinkSync(filePath);
    console.log("File deleted: ".concat(filePath));
}
function main() {
    var _a = process.argv.slice(2), command = _a[0], source = _a[1], destination = _a[2];
    if (!command || !source) {
        console.log('usage: file-tool <command> <source> [destination]');
        console.log('Commands: copy, move, delete');
        return;
    }
    switch (command.toLowerCase()) {
        case 'copy':
            if (!destination) {
                console.log('Destination path is required for copy command.');
            }
            copyFile(source, destination);
            break;
        case 'move':
            if (!destination) {
                console.log('Destinaton path is required for move command.');
                return;
            }
            moveFile(source, destination);
            break;
        case 'delete':
            deleteFile(source);
            break;
        default:
            console.log('Invalid command. Usage: file-tool <command> <source> [destination]');
            console.log('Commands: copy, move, delete');
    }
}
main();
