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

	if port != nil {
		c.Port = *porrt
	}
	if timeout != nil {
		c.Timeout = *timeout
	}

	return c
}

func main() {
	port := 9090
	timeout := 120

	config1 := NewConfig(nil, nil)
	fmt.Printf("Config1:  Port=%d, Timeout=%d\n", config1.Port, config1.Timeout)

	config2 := NewConfig(&port, nil)
	fmt.Printf("Config2: Port=%d, Timeout=%d\n", config2.Port, config2.Timeout)

	config3 := NewConfig(&port, &timeout)
	fmt.Printf("Config3: Port=%d, Timeout=%d\n", config3.Port, config3.Timeout)
}
