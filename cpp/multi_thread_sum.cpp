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

	for (int i = 0; i < num_threads; ++i) {
		int start = i * part;
		int end = (i == num_threads -1) ? size : start + part;
		threads.push_back(std::thread(partial_sum, std::ref(numbers), start, end, std::ref(results[i])));
	}

	for (auto& thread : threads) {
		thread.join();
	}

	int total_sum = std::accumulate(results.begin(), results.end(), 0);

	std::cout << "Total sum: " << total_sum << std::endl;

	return 0;
}
