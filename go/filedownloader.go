package main

import (
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
)

func main() {
	var url string
	flag.StringVar(&url, "url", "", "URL of the file to download")
	flag.Parse()

	if url == "" {
		fmt.Println("Please provide a URL using the -url flag")
		return
	}

	fileName := filepath.Base(url)

	file, err := os.Create(fileName)
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()

	response, err := http.Get(url)
	if err != nil {
		fmt.Println("Error downloading file:", err)
		return
	}
	defer response.Body.Close()

	_, err = io.Copy(file, response.Body)
	if err != nil {
		fmt.Println("Error writing to file:", err)
		return
		
	}
}
