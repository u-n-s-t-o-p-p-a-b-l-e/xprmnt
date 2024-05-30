#include <iostream>
#include <vector>
#include <string>
#include <iomanip>

struct Expense {
	std::string description;
	double amount;
};

void addExpense(std::vector<Expense>& expenses, const std::string& description, double amount) {
	Expense newExpense = {description, amount};
	expenses.push_back(newExpense);
	std::cout << "Expense added: " << description << " ($" << std::fixed << std::setprecision(2) << amount << ")" << std::endl;
}

void viewExpenses(const std::vector<Expense>& expenses) {
	if (expenses.empty()) {
		std::cout << "No expenses recorded." << std::endl;
	} else {
		std::cout << "Expenses:\n";
		double total = 0.0;
		for (const auto& expense : expenses) {
			std::cout << expense.description << ": $" << std::fixed << std::setprecision(2) << expense.amount << std::endl;
			total += expense.amount;
		}
		std::cout << "Total expenses: $" << std::fixed << std::setprecision(2) << total << std::endl;
	}
}

int main() {
	std::vector<Expense> expenses;

	while (true) {
		std::cout << "\nExpense Tracker\n";
		std::cout << "1. Add Expense\n";
		std::cout << "2. View Expenses\n";
		std::cout << "3. Exit\n";
		std::cout << "Enter your choice: ";

		int choice;
		std::cin >> choice;

		switch (choice) {
			case 1: {
						std::cin.ignore();
						std::string description;
						double amount;
						std::cout << "Enter expense description: ";
						std::getline(std::cin, description);
						std::cout << "Enter expense amount: $";
						std::cin >> amount;
						addExpense(expenses, description, amount);
						break;
					}
			case 2:
					viewExpenses(expenses);
					break;
			case 3:
					std::cout << "Exiting...\n";
					return 0;
			default:
					std::cout << "Invalid choice. Please try again.\n";
		}
	}

	return 0;
}
