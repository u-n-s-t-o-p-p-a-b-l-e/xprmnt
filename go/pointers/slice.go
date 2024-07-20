package main

import "fmt"

func doubleValues(nums *[]int) {
	for i := range *nums {
		(*nums)[i] *= 2
	}
}
