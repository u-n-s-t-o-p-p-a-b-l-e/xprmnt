package main

import "fmt"

type Database interface {
	GetData() string
}

type RealDatabase struct{}
