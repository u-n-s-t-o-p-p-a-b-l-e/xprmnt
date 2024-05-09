#include <iostream>

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
		std::cout << "Error: Cannot divide by zero\n";
		return 0;
	}
	return a / b;
}

int main() {
	char operation;
	double num1, num2;

	std::cout << "Enter operation (+, -, *, /): ";
	std::cin >> operation;

	std::cout << "Enter two numbers: ";
	std::cin >> num1 >> num2;

	switch(operation) {
		case '+':
			std::cout << "Result: " << add(num1, num2 ) << std:: endl;
			break;
		case '-':
			std::cout << "Result: " << substract(num1, num2 ) << std:: endl;
			break;
		case '*':
			std::cout << "Result: " << multiply(num1, num2 ) << std:: endl;
			break;
		case '/':
			std::cout << "Result: " << divide(num1, num2 ) << std:: endl;
			break;
		default: 
			std::cout << "Invalid Operation\n";
	}

	return 0;
}
