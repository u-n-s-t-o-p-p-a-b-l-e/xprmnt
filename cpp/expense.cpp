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
