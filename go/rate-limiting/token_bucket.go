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

func NewTokenBucket(rate, max int) *TokenBucket {
	tb := &TokenBucket{
		rate: rate,
		max:   max,
		bucket: max,
		ticker: time.NewTicker(1 * time.second),
		quit: make(chan struct{}),
	}
	go tb.refill()
	return tb
}
