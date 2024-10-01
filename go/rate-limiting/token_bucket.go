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
		rate:   rate,
		max:    max,
		bucket: max,
		ticker: time.NewTicker(1 * time.Second),
		quit:   make(chan struct{}),
	}
	go tb.refill()
	return tb
}

func (tb *TokenBucket) refill() {
	for {
		select {
		case <-tb.ticker.C:
			if tb.bucket < tb.max {
				tb.bucket += tb.rate
				if tb.bucket > tb.max {
					tb.bucket = tb.max
				}
				fmt.Printf("Bucket refilled: %d tokens available\n", tb.bucket)
			}
		case <-tb.quit:
			return
		}
	}
}

func (tb *TokenBucket) Allow() bool {
	if tb.bucket > 0 {
		tb.bucket--
		return true
	}
	return false
}
