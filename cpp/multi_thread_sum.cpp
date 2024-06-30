#include <iostream>
#include <vector>
#include <thread>
#include <numeric>
#include <functional>

void partial_sum(const std::vector<int>& numw, int start, int end, int& result) {
	result = std::accumulate(nums.begin() + start, nums.begin() + end, 0);
}
