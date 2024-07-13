package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func celciusToFahrenheit(celcius float64) float64 {
	return (celcius * 5 / 9) + 32
}

func fahrenheitToCelcius(fahrenheit float64) float64 {
	return (fahrenheit - 32) * 5 / 9
}

func main() {
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("Temperatur Converter")
	fmt.Println("1. Celcius to Fahrenheit")
	fmt.Println("2. Fahrenheit to Celcius")
	fmt.Print("Enter your choice (1 or 2): ")
	choiceStr, _ := reader.ReadString('\n')
	choice, err := strconv.Atoi(choiceStr[:len(choiceStr)-1])
	if err != nil || (choice != 1 && choice != 2) {
		fmt.Println("Invalid choice. Please enter 1 or 2.")
		return
		
	}

	var inputUnit, outputUnit string
	var convert func(float64) float64

	if choice == 1 {
		inputUnit = "Celcius"
		outputUnit = "Fahrenheit"
		convert = celciusToFahrenheit
	} else {
		inputUnit = "Fahrenheit"
		outputUnit = "Celcius"
		convert = fahrenheitToCelcius
	}

	fmt.Printf("Enter the temperature in %s: ", inputUnit)
	tempStr, _ := reader.ReadString('\n')
	temp, err := strconv.ParseFloat(tempStr[:len(tempStr)-1], 64)
	if err != nil {
		fmt.Println("Invalid input. Please enter a valid temperature.")
		return
		
	}

	result := convert(temp)
	fmt.Printf("The temperature %.2f %s is %.2f %s\n", temp, inputUnit, result, outputUnit)
	
	
	
	
	
}
