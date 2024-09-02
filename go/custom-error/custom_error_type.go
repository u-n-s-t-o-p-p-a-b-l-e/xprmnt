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

func main() {
	err := validateName("")
	if err != nil {
		fmt.Println("Error:", err)
	}

	if vErr, ok := err.(*ValidationError); ok {
		fmt.Println("Field:", vErr.Field)
		fmt.Println("Message:", vErr.Message)
	}
}
