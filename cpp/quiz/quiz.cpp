#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <random> // For std::shuffle
#include <algorithm> // For std::shuffle
#include <ctime>

struct Question {
    std::string questionText;
    std::string correctAnswer;
};

// Function to read questions from a file
std::vector<Question> readQuestionsFromFile(const std::string& filename) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        std::cerr << "Error: Unable to open file: " << filename << std::endl;
        exit(1);
    }

    std::vector<Question> questions;
    std::string line;
    while (std::getline(file, line)) {
        Question q;
        q.questionText = line;
        if (std::getline(file, line)) {
            q.correctAnswer = line;
            questions.push_back(q);
        }
    }

    file.close();
    return questions;
}

// Function to shuffle the order of questions
void shuffleQuestions(std::vector<Question>& questions) {
    static std::random_device rd;
    static std::mt19937 g(rd());
    std::shuffle(questions.begin(), questions.end(), g);
}

// Function to play the quiz
void playQuiz(const std::vector<Question>& questions) {
    int score = 0;
    for (const auto& q : questions) {
        std::cout << "Question: " << q.questionText << std::endl;
        std::string userAnswer;
        std::cout << "Your answer: ";
        std::getline(std::cin, userAnswer);

        if (userAnswer == q.correctAnswer) {
            std::cout << "Correct!\n";
            score++;
        } else {
            std::cout << "Incorrect! The correct answer is: " << q.correctAnswer << std::endl;
        }
    }

    std::cout << "Quiz completed!\n";
    std::cout << "Your score: " << score << "/" << questions.size() << std::endl;
}

int main() {
    std::string filename = "quiz_questions.txt"; // Change this to your filename
    std::vector<Question> questions = readQuestionsFromFile(filename);
    shuffleQuestions(questions);

    std::cout << "Welcome to the Quiz Game!\n";
    std::cout << "You will be presented with a series of questions.\n";
    std::cout << "Enter your answer for each question.\n";

    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n'); // Clear the input buffer before starting the quiz

    playQuiz(questions);

    return 0;
}

