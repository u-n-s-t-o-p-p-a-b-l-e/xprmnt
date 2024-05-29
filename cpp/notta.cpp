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

void viewNotes(const std::vector<Note>& notes) {
	if (notes.empty()) {
		std::cout << "No notes recorded." << std::endl;
	} else {
		std::cout << "Notes:\n";
		for (const auto& note : notes) {
			std::cout << "Title: " << note.title << "\n";
			std::cout << "Content: " << note.content << "\n\n";
		}
	}
}

void searchNotes(const std::vector<Note>& notes, const std::string& keyword) {
	std::cout << "Search results for keyword '" << keyword << "':\n";
	bool found = false;
	for (const auto& note : notes) {
		if (note.title.find(keyword) != std::string::npos || note.content.find(keyword) != std::string::npos) {
			std::cout << "Title: " << note.title << "\n";
			found = true;
		}
	}
}
