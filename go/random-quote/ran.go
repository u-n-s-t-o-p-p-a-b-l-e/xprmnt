package main

import (
	"fmt"
	"math/rand"
	"time"
)

var quotes = []string{
	"Be yourself; everyone else is already taken. - Oscar Wilde",
	"In three words I can sum up everything I've learned about life: it goes on. - Robert Frost",
	"The only way to do great work is to love what you do. - Steve Jobs",
	"Be the change that you wish to see in the world. - Mahatma Gandhi",
	"Live as if you were to die tomorrow. Learn as if you were to live forever. - Mahatma Gandhi",
}

func main() {
	rand.Seed(time.Now().UnixNano())

	randomIndex := rand.Intn(len(quotes))

	fmt.Println("Random Quote: ")
	fmt.Println(quotes[randomIndex])
	
	
}
