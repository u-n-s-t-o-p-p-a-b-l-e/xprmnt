package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	fmt.Print("Enter the first number: ")
	num1Str, _ := reader.ReadString('\n')
	num1, err := strconv.ParseFloat(num1Str[:len(num1Str)-1], 64)
	if err != nil {
		fmt.Println("Invalid input. Please enter a valid number.")
		return
	}
	fmt.Print("Enter the second number: ")
	num2Str, _ :=  reader.ReadString('\n')
	num2, err := strconv.ParseFloat(num2Str[:len(num2Str)-1], 64)
	if err != nil {
		fmt.Println("Invalid input. Please enter a valid number.")
		return

	}

	sum := num1 + num2
	fmt.Printf("The sum of %.2f and %.2f is %.2f\n", num1, num2, sum)
}
