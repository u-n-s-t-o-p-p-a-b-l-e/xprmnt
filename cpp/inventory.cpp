#include <iostream>
#include <map>
#include <string>

void addItem(std::map<std::string, int>& inventory, const std::string& item, int quantity) {
	inventory[item] += quantity;
	std::cout << "Added " << quantity << " " << item << "(s) to inventory." << std::endl;
}

void removeItem(std::map<std::string, int>& inventory, const std::string& item, int quantity) {
	if (inventory.find(item) != inventory.end() && inventory[item] >= quantity) {
		inventory[item] -= quantity;
		std::cout << "Removed " << quantity << " " << "(s) from inventory." << std::endl;
	} else {
		std::cout << "Error: Insufficient quantity of " << item << " in inventory." << std::endl;
	}
}
