package main

import "fmt"

func modifyElement(arr []int, index int, newVal int) {
	if index >= 0 && index < len(arr) {
		arr[index] = newVal
	} else {
		fmt.Println("Index out of bounds!")
	}
}
