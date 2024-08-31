package main

import (
	"context"
	"errors"
	"fmt"
	"reflect"
	"sync"
	"time"
)

type Processor interface {
	Process(ctx context.Context, data interface{}) (interface{}, error)
}

type Pipeline struct {
	processors []Processor
}

func NewPipeling(processors ...Processor) *Pipeline {
	return &Pipeline{processors: processors}
}

func (p *Pipeline) Execute(ctx context.Context, data interface{}) (interface{}, error) {
	var result interface{} = data
	var err error

	for _, processor := range p.processors {
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		default:
			result, err = processor.Process(ctx, result)
			if err != nil {
				return nil, fmt.Errorf("Pipeline execution error: %w", err)
			}
		}
	}

	return result, nil
}

type StringToIntProcessor struct{}

func (p *StringToIntProcessor) Process(ctx context.Context, data interface{}) (interface{}, error) {
	str, ok := data.(string)
	if !ok {
		return nil, errors.New("input is not a string")
	} 

	var result int
	for _, char := range str {
		if char < '0' || char > '9' {
			return nil, errors.New("input contains non-digit characters")
		}
		result = result*10 + int(char-'0')
	}

	return result, nil
}


