package main

import (
	"fmt"
)

type ValidationError struct {
	Field   string
	Message string
}

func (e *ValidationError) Error() string {
	return fmt.Sprintf("validation error: field '%s' - %s", e.Field, e.Message)
}

func validateName(name string) error {
	if name == "" {
		return &ValidationError{
			Field: "Name",
			Message: "cannot be empty",
		}
	}
	return nil
}
