#!/usr/bin/env node
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
var notesFilePath = 'notes.json';
function loadNotes() {
    try {
        var data = fs.readFileSync(notesFilePath, 'utf-8');
        return JSON.parse(data);
    }
    catch (error) {
        return [];
    }
}
function saveNotes(notes) {
    fs.writeFileSync(notesFilePath, JSON.stringify(notes, null, 2));
}
function addNote(title, body) {
    var notes = loadNotes();
    notes.push({ title: title, body: body });
    saveNotes(notes);
    console.log('Note added succesfully');
}
function listNotes() {
    var notes = loadNotes();
    console.log('Your Notes:');
    notes.forEach(function (note, index) {
        console.log("".concat(index + 1, ", Title: ").concat(note.title));
        console.log(" Body: ".concat(note.body));
    });
}
function removeNotesByTitle(title) {
    var notes = loadNotes();
    var initialLength = notes.length;
    notes = notes.filter(function (note) { return note.title !== title; });
    if (notes.length === initialLength) {
        console.log("Note with title \"".concat(title, "\" not found."));
    }
    else {
        saveNotes(notes);
        console.log("Note with title \"".concat(title, "\" removed succesfully"));
    }
}
function main() {
    var _a = process.argv.slice(2), command = _a[0], args = _a.slice(1);
    switch (command) {
        case 'add':
            var title = args[0], body = args.slice(1);
            addNote(title, body.join(' '));
            break;
        case 'list':
            listNotes();
            break;
        case 'remove':
            var noteTitle = args.join(' ');
            removeNotesByTitle(noteTitle);
            break;
        default:
            console.log('Invalid command. Usage: notes add|list|remove');
    }
}
main();
