package main

import "fmt"

type Config struct {
	Port	int
	Timeout int
}

func NewConfig(port *int, timeout *int) *Config {
	c := &Config{
		Port: 8080,
		Timeout: 60,
	}
}
