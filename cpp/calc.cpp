#include <iostream>
#include <string>
#include <sstream>

double add(double a, double b) {
	return a + b;
}

double substract(double a, double b) {
	return a - b;
}

double multiply(double a, double b) {
	return a * b;
}

double divide(double a, double b) {
	if (b == 0) {
		std::cerr << "Error: Division by zero\n";
		return 0;
	}
	return a / b;
}

int main() {
	std::string input;
	double result = 0.0;
	char op;

	std::cout << "Welcome to Simple Calculator!\n";
	std::cout << "Enter expression (e.g., 2 + 3 * 4 / 2 - 1) or 'quit' to exit:\n";

	while (true) {
		std::getline(std::cin, input);
		if (input == "quit")
			break;

		std::istringstream iss(input);
		double num;
		iss >> result;

		while (iss >> op >> num) {
			switch (op) {
				case '+':
					result = add(result, num);
					break;
				case '-':
					result = substract(result, num);
					break;
				case '*':
					result = multiply(result, num);
					break;
				case '/':
					result = divide(result, num);
					break;
				default:
					std::cerr << "Error: Invalid operator '" << op << "'\n";
					break;
			}
		}

		std::cout << "Result: " << result << std::endl;;
	}

	std::cout << "Goodbye!\n";
	return 0;
}


