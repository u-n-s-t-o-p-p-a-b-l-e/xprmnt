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
