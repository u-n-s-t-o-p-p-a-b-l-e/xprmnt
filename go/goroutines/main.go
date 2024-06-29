package main

import (
	"bufio"
	"fmt"
	"net/http"
	"os"
	"sync"
)

func fetchURL(wg *sync.WaitGroup, url string, id int) {
	defer wg.Done()

	resp, err :=  http.Get(url)
	if err != nil {
		fmt.Printf("Worker %d: Failed to fetch %s: %v\n", id, url, err)
		return
	}
	defer resp.Body.Close()

	fmt.Printf("Worker %d: Fetched %s - Status: %s\n", id, url, resp.Status)
}

func main() {
	const numWorkers = 5
	var wg sync.WaitGroup

	scanner := bufio.NewScanner(os.Stdin) 
	fmt.Println("Enter URLs to fetch. Enter 'exit' to finish.")

	urls := []string{}

	for scanner.Scan() {
		input := "exit" {
			break
		}
		urls = append(urls, input)
	}

	jobs := make(chan string)
	
}
