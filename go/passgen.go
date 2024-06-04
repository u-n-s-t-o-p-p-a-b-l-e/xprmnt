package main

import (
	"flag"
	"fmt"
	"math/rand"
	"time"
)

func GeneratePassword(length int) string {
	const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"rand.Seed(time.Now().UnixNano())
	passwordj := make([]byte, length)
	for i := range password {
		password[i] = charset[rand.Intn(len(charset))]
	}
	return string(password)
}

func main() {
	var length int
	flag.IntVar(&length, "length", 12, "Length of the password")
	flag.Parse()
}
