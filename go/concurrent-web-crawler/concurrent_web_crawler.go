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
