package main

import "fmt"

func modifyElement(arr []int, index int, newVal int) {
	if index >= 0 && index < len(arr) {
		arr[index] = newVal
	} else {
		fmt.Println("Index out of bounds!")
	}
}

func main() {
	arr := []int{1, 2, 3, 4, 5}

	fmt.Println("Before modification:", arr)
	modifyElement(arr, 2, 10)
	fmt.Println("After modification:", arr)
}
