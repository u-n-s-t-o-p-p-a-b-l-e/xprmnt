#include <iostream>
#include <map>
#include <string>

struct Contact {
	std::string name;
	std::string phoneNumber;
};

void addContact(std::map<std::string, Contact>& contacts, const std::string& name, const std::string& phoneNumber) {
	Contact newContact = {name, phoneNumber};
	contacts[name] = newContact;
	std::cout << "Contact added: " << name << " (" << phoneNumber << ")" << std::endl;
}

void viewContacts(const std::map<std::string, Contact>& contacts) {
	std::cout << "Contacts:\n";
	for (const auto& pair: contacts) {
		std::cout << "Name: " << pair.second.name << ", Phone Number: " << pair.second.phoneNumber << std::endl;
	}
}

void searchContact(const std::map<std::string, Contact>& contacts, const std::string& name) {
	auto it = contacts.find(name);
	if (it != contacts.end()) {
		std::cout << "Name: " << it->second.name << ", Phone Number: " << it->second.phoneNumber << std::endl;
	} else {
		std::cout << "Contact not found." << std::endl;
	}
}

int main() {
	std::map<std::string, Contact> contacts;

	while (true) {
		std::cout << "\nContact Management System\n";
		std::cout << "1. Add Contact\n";
		std::cout << "2. View Contacts\n";
		std::cout << "3. Search Contact\n";
		std::cout << "4. Exit\n";
		std::cout << "Enter your choice: ";

		int choice;
		std::cin >> choice;

		switch (choice) {
			case 1: {
						std::string name, phoneNumber;
						std::cout << "Enter contact name: ";
						std::cin >> name;
						std::cout << "Enter contact phone number: ";
						std::cin >> phoneNumber;
						addContact(contacts, name, phoneNumber);
						break;
					}
			case 2:
					viewContacts(contacts);
					break;
			case 3: {
						std::string name;
						std::cout << "Enter name to search: ";
						std::cin >> name;
						searchContact(contacts, name);
						break;
					}
			case 4:
					std::cout << "Exiting...\n";
					return 0;
			default:
					std::cout << "Invalid choice. Please try again.\n";
		}
	}

	return 0;

}
