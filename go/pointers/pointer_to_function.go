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

func main() {
	num := 10
	fmt.Println("Before modifyValue:", num)
	modifyValue(&num)
	fmt.Println("After modifyValue:", num)

	nums := []int{1, 2, 3, 4, 5}
	fmt.Println("Before applyFunction:", nums)
	applyFunction(nums, modifyValue)
	fmt.Println("After applyFunction:", nums)
}
