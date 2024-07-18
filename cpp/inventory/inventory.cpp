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

void viewInventory(const std::map<std::string, int>& inventory) {
	std::cout << "Current Inventory:\n";
	for (const auto& item : inventory) {
		std::cout << item.first << ": " << item.second << std::endl;
	}
}

int main() {
	std::map<std::string, int> inventory;

	while (true) {
		std::cout << "\nInventory Management System\n";
		std::cout << "1. Add Item\n";
		std::cout << "2. Remove Item\n";
		std::cout << "3. View Inventory\n";
		std::cout << "4. Exit\n";
		std::cout << "Enter your choice: ";

		int choice;
		std::cin >> choice;

		switch (choice) {
			case 1: {
						std::string item;
						int quantity;
						std::cout << "Enter item name: ";
						std::cin >> item;
						std::cout << "Enter quantity to add: ";
						std::cin >> quantity;
						addItem(inventory, item, quantity);
						break; 
					}
			case 2: {
						std::string item;
						int quantity;
						std::cout << "Enter item name: ";
						std::cin >> item;
						std::cout << "Enter quantity to remove: ";
						std::cin >> quantity;
						removeItem(inventory, item, quantity);
						break;
					}
			case 3:
					viewInventory(inventory);
					break;
			case 4:
					std::cout << "Exiting...\n";
					return 0;
			default:
					std::cout << "Invalid choice. Please try again.\n";
		}
	}

	return 0;
}
