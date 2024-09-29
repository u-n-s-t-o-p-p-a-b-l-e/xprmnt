package main

import (
	"fmt"
	"time"
)

type TokenBucket struct {
	rate   int
	bucket int
	max    int
	ticker *time.Ticker
	quit chan struct{}
}
