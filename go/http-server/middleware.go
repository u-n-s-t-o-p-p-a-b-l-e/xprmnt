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

func LongRunningHandler(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	select {
	case <-time.After(5 * time.Second):
		fmt.Println(w, "Work completed succesfully")
	case <-ctx.Done():
		http.Error(w, "Request canceled or timed out", http.StatusRequestTimeout)
	}
}

func main() {
	mux := http.NewServeMux()

	mux.Handle("/work", TimeoutMiddleware(2*time.Second, http.HandlerFunc(LongRunningHandler)))

	loggingMux := LoggingMiddleware(mux)

	server := &http.Server{
		Addr: ":8080",
		Handler: loggingMux,
	}

	log.Println("Starting server on :8080")
	if err := server.ListenAndServe(); err != nil {
		log.Fatalf("Server failed: %s", err)
	}
}
