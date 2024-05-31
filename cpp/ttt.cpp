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

bool checkWin(const std::vector<std::vector<char>>& board, char player) {
	for (int i = 0; i < 3; ++i) {
		if ((boaard[i][0] ==  player && board[i][1] == player && board[i][2] == player) || (board[0][i] ==  player && board[1][i] == player && board[2][i] == player)) {
			return true;
		}
	}
	if ((board[0][0] ==  player && board[1][1] ==  player && board[2][2] == player) || (board[0][2] == player && board[1][1] == player && board[2][0] == player)) {
		return true;
	}
	return false;
}

bool checkDraw(const std::vector<std::vector<char>>& board) {
	for (const auto& row : board) {
		for (char cell : row) {
			if (cell == ' ') return false;
		}
	}
	return true;
}

void makeMove(std::vector<std::vector<char>>& board, int row, int col, char player) {
	if (board[row][col] == ' ') {
		board[row][col] = player;
	} else {
		std::cout << "Cell already occupied! Choose a different cell.\n";
	}
}

int main() {
	std::vector<std::vector<char>> board(3, std::vector<char>(3, ' '));
	char currentPlayer = 'X';
	bool gameWon = false;
	bool gameDraw = false;

	std::cout << "Welcome to Tic-Tac-Toe!\n";
	printBoard(board);
}
