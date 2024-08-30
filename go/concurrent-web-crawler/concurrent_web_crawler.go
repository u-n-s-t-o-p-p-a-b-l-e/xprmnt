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
}
