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

func NewPipeline(processors ...Processor) *Pipeline {
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

type DoubleIntProcessor struct{}

func (p *DoubleIntProcessor) Process(ctx context.Context, data interface{}) (interface{}, error) {
	num, ok := data.(int)
	if !ok {
		return nil, errors.New("input is not an int")
	}

	return num * 2, nil
}

type GenericProcessor struct{}

func (p *GenericProcessor) Process(ctx context.Context, data interface{}) (interface{}, error) {
	value := reflect.ValueOf(data)

	switch value.Kind() {
	case reflect.Int:
		return value.Int() + 1, nil
	case reflect.String:
		return value.String() + "!", nil
	default:
		return nil, fmt.Errorf("unsupported type: %v", value.Type())
	}
}

type AsyncProcessor struct {
	processor Processor
}

func NewAsyncProcessor(p Processor) *AsyncProcessor {
	return &AsyncProcessor{processor: p}
}

func (p *AsyncProcessor) Process(ctx context.Context, data interface{}) (interface{}, error) {
	resultChan := make(chan interface{})
	errChan := make(chan error)

	go func() {
		result, err := p.processor.Process(ctx, data)
		if err != nil {
			errChan <- err
		} else {
			resultChan <- result
		}
	}()

	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	case err := <-errChan:
		return nil, err
	case result := <-resultChan:
		return result, nil
	}
}

func main() {
	pipeline := NewPipeline(
		&StringToIntProcessor{},
		NewAsyncProcessor(&DoubleIntProcessor{}),
		&GenericProcessor{},
	)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	result, err := pipeline.Execute(ctx, "42")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	fmt.Printf("Result %v\n", result)

	var wg sync.WaitGroup
	inputs := []string{"10", "20", "30"}

	for _, input := range inputs {
		wg.Add(1)
		go func(in string) {
			defer wg.Done()
			res, err := pipeline.Execute(ctx, in)
			if err != nil {
				fmt.Printf("Error processing %s: %v\n", in, err)
			} else {
				fmt.Printf("Result for %s: %v\n", in, res)
			}
		}(input)
	}

	wg.Wait()
}
