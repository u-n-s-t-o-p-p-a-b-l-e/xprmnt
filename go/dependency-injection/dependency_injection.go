package main

import "fmt"

type Database interface {
	GetData() string
}
