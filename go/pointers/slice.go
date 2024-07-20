package main

import "fmt"

func doubleValues(nums *[]int) {
	for i := range *nums {
		(*nums)[i] *= 2
	}
}

func main() {
	nums := []int{1, 2, 3, 4, 5}

	fmt.Println("Before doubling:", nums)
	doubleValues(&nums)
	fmt.Println("After doubling:", nums)
}


