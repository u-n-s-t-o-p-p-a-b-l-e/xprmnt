#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

struct Note {
	std::string title;
	std::string content;
};

void addNote(std::vector<Note>& notes, const std::string& title, const std::string& content) {
	Note newNote = {title, content};
	notes.push_back(newNote);
	std::cout << "Note added: " << title << std::endl;
}
