#!/usr/bin/env node

import * as fs from 'fs';

interface Note {
	title: string;
	body: string;
}

const notesFilePath = 'notes.json';

function loadNotes(): Note[] {
	try {
		const data = fs.readFileSync(notesFilePath , 'utf-8');
		return JSON.parse(data);
	} catch (error) {
		return [];
	}
}

function saveNotes (notes: Note[]): void {
	fs.writeFileSync(notesFilePath, JSON.stringify(notes, null, 2));
}

function addNote (title: string, body: string): void {
	const notes = loadNotes();
	notes.push({ title, body });
	saveNotes(notes);
	console.log('Note added succesfully');
}

function listNotes (): void {
	const notes = loadNotes();
	console.log('Your Notes:');
	notes.forEach((note, index) => {
		console.log(`${index + 1}, Title: ${note.title}`);
		console.log(` Body: ${note.body}`); 	
	});
}

function removeNotesByTitle (title: string): void {
	let notes = loadNotes();
	const initialLength = notes.length;
	notes = notes.filter(note => note.title !== title);
	if (notes.length === initialLength) {
		console.log(`Note with title "${title}" not found.`);
	} else {
		saveNotes(notes);
		console.log(`Note with title "${title}" removed succesfully`);
	}
}

function main(): void {
	const [command, ...args] = process.argv.slice(2);

	switch (command) {
		case 'add':
			const [title, ...body] = args;
		addNote(title, body.join(' '));
		break;
		case 'list':
			listNotes();
		break;
		case 'remove':
			const noteTitle = args.join(' ');
		removeNotesByTitle(noteTitle);
		break;
		default:
			console.log('Invalid command. Usage: notes add|list|remove');
	}
}

main();
