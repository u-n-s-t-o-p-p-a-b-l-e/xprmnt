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
}
