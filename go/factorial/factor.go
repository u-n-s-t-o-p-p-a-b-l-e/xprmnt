package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func factorial(n int) int {
	if n == 0 {
		return 1
	}

	return n * factorial(n-1)
}

func main() {
	reader := bufio.NewReader(os.Stdin)

	fmt.Print("Enter a number to calculate its factorial: ")
	input, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println("Error reading input: ", err)
		return
		
	}

	num, err := strconv.Atoi(input[:len(input)-1])
	if err != nil {
		fmt.Println("Invalid input. Please enter a valid number.")
	}

	result := factorial(num) 
	fmt.Println("The factorial of %d is : %d\n", num, result)
}
