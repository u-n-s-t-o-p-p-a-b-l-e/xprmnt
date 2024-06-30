#include <iostream>
#include <vector>
#include <thread>
#include <numeric>
#include <functional>

void partial_sum(const std::vector<int>& nums, int start, int end, int& result) {
	result = std::accumulate(nums.begin() + start, nums.begin() + end, 0);
}

int main() {
	std::vector<int> numbers;
	for (int i = 1; i <= 100; ++i) {
		numbers.push_back(i);
	}

	int num_threads = 4;
	int size = numbers.size();
	int part = size / num_threads;

	std::vector<int> results(num_threads, 0);
	std::vector<std::thread> threads;
}
