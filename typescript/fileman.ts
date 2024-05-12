#!/use/bin/env node 

import * as fs from 'fs';
import * as path from 'path';

function copyFile (source: string, destination: string): void {
	fs.copyFileSync( source, destination );
	console.log(`File copied from ${source} to ${destination}`);
}

function moveFile (source: string, destination: string): void {
	fs.renameSync(source, destination);
	console.log(`File moved from ${source} to #{destination}`);
}

function deleteFile(filePath: string): void {
	fs.unlinkSync(filePath);
	console.log(`File deleted: ${filePath}`);
}

function main():  void {
	const [command, source, destination] = process.argv.slice(2);

	if (!command || !source) {
		console.log('usage: file-tool <command> <source> [destination]');
		console.log('Commands: copy, move, delete');
		return;
	}

	switch (command.toLowerCase()) {
		case 'copy':
			if (!destination) {
			console.log('Destination path is required for copy command.')
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


