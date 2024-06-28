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
}
