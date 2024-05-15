package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("Enter numbers separated by spaces (e.g., 1 2 3)")
	input, _ = reader.ReadString('\n')

	numStrings := strings.Fields(input)

	sum := 0
	for _, numStr := range numStrings {
		num, err := strconv.Atoi(numStr)
		if err != nil {
			fmt.Print("Invalid input: '%s' is not a valid number\n", numStr)
			return;
		}
		sum +=  num
	}

	fmt.Print("Sum of the numbers is: %d\n", sum)
	
	
}


