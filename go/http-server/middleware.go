package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"
)

func LoggingMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		log.Printf("Started %s %s", r.Method, r.URL.Path)

		next.ServeHTTP(w, r)

		log.Printf("Completed %s %s in %v", r.Method, r.URL.Path, time.Since(start))
	})
}

func TimeoutMiddleware(timeout time.Duration, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx, cancel := context.WithTimeout(r.Context(), timeout)

		defer cancel()

		r = r.WithContext(ctx)

		next.ServeHTTP(w, r)

		if ctx.Err() == context.DeadlineExceeded {
			log.Printf("Request %s %s timed out", r.Method, r.URL.Path)
		}
	})
}
