package main

import "fmt"

func modifyValue(val *int) {
	*val = 42
}

func applyFunction(nums []int, f func(*int)) {
	for i := range nums {
		f(&nums[i])
	}
}
