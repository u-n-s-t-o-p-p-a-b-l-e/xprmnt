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

type pipeline struct {
	processors []Processor
}
