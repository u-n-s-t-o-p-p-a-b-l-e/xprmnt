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

void shuffleQuestions(std::vector<Question>& questions) {
	static std::random_device rd;
	static std::mt19937 g(rd());
	std::shuffle(questions.begin(), questions.end(), g);
}

void playQuiz(const std::vector<Question>& questions) {
	int score = 0;
	for (const auto& q : questions) {
		std::cout << "Question: " << q.questionText << std::endl;
		std::string userAnswer;
		std::cout << "Your answer: ";
		std::cin.ignore();
		std::getline(std::cin, userAnswer);

		if (userAnswer == q.correctAnswer) {
			std::cout << "Correct!\n";
			score++;
		} else {
			std::cout << "Incorrect! The correct answer is: " << q.correctAnswer << std::endl;
		}
	}
}
