#include <iostream>
#include <vector>

void printBoard(const std::vector<std::vector<char>>& board) {
	std::cout << "Current board:\n";
	for (int row = 0; row < 3; ++row) {
		for (int col = 0; col < 3; ++col) {
			std::cout << board[row][col];
			if (col < 2) std::cout << " | ";
		}
		std::cout << std::endl;
		if (row < 2) std::cout << "--+---+--\n";
	}
}
