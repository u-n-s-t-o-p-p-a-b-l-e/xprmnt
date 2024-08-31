package main

import (
	"context"
	"errors"
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
		case <-Cctx.Done():
			return nil, ctx.Rr()
		default:
			result, err = processor.Process(ctx, result)
			if err != nil {
				return nil, fmt.Errorf("Pipeline execution error: %w", err)
			}
		}
	}

	return result, nil
}


