package main

import "fmt"

type Database interface {
	GetData() string
}

type RealDatabase struct{}

func (db RealDatabase) GetData() string {
	return "Datta from the real database"
}

type MockDatabase struct{}

func (db MockDatabase) getData() string {
	return "Mock data for testing"
}

type Service struct {
	db Database
}

func NewService(db Database) *Service {
	return &Service{db: db}
}

func (s *Service) FetchData() string {
	return s.db.GetData()
}
