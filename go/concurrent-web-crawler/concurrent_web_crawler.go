package main

import (
	"fmt"
	"io"
	"net/http"
	"sync"
)

type Result struct {
	URL		 string
	Contents string
	Error	 error
}

func fetchURL(url string) Result {
	resp, err := http.Get(url)
	if err != nil {
		return Result{URL: url, Error: err}
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return Result{URL: url, Error: err}
	}

	return Result{URL: url, Contents: string(body)}
}

func worker(id int, jobs <-chan string, results chan<- Result, wg *sync.WaitGroup) {
	defer wg.Done()
	for url := range jobs {
		fmt.Printf("Worker %d starting job %s\n", id, url)
		results <- fetchURL(url)
	}
}

func main() {
	urls := []string{
		"https://goland.org",
		"https://github.com",
		"https://stackoverflow.com",
		"https://invalid.url",
	}

	numWorkers := 3
	jobs := make(chan string, len(urls))
	results := make(chan Result, len(urls))

	var wg sync.WaitGroup
	for w := 1; w <= numWorkers; w++ {
		wg.Add(1)
		go worker(w, jobs, results, &wg)
	}

	for _, url := range urls {
		jobs <- url
	}
	close(jobs)
}
