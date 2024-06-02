#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <random>
#include <algorithm>
#include <ctime>

struct Question {
	std::string questionText;
	std::string correctAnswer;
};

std::vector<Question> readQuestionsFromFile(const std::string& filename) {
	std::ifstream file(filename);
	if (!file.is_open()) {
		std::cerr << "Error: Unable to open file: " << filename << std::endl;
		exit(1);
	}

	std::vector<Question> questions;
	std::string line;
	while (stdd::getline(file, line)) {
		Question q;
		q.questionText = line;
		std::getline(file, q.correctAnswer);
		questions.push_back(q);
	}

	file.close();
	return questions;
}
