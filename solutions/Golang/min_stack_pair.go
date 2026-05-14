package main

import (
	"fmt"
)

/**
 * Min Stack using single stack with (value, min) pairs.
 *
 * Each element in the stack is a struct of (value, min_at_this_level).
 * The min_at_this_level is the minimum value from the bottom up to
 * and including the current element.
 *
 * Time: O(1) per operation
 * Space: O(n)
 */
type Element struct {
	value int
	min   int
}

type MinStack struct {
	stack []Element
}

func NewMinStack() *MinStack {
	return &MinStack{
		stack: []Element{},
	}
}

func (ms *MinStack) Push(val int) {
	if len(ms.stack) == 0 {
		// First element: value is the minimum
		ms.stack = append(ms.stack, Element{value: val, min: val})
	} else {
		// Store both value and the minimum up to this point
		currentMin := val
		if ms.stack[len(ms.stack)-1].min < val {
			currentMin = ms.stack[len(ms.stack)-1].min
		}
		ms.stack = append(ms.stack, Element{value: val, min: currentMin})
	}
}

func (ms *MinStack) Pop() {
	ms.stack = ms.stack[:len(ms.stack)-1]
}

func (ms *MinStack) Top() int {
	return ms.stack[len(ms.stack)-1].value
}

func (ms *MinStack) GetMin() int {
	return ms.stack[len(ms.stack)-1].min
}

func main() {
	stack := NewMinStack()
	stack.Push(-2)
	stack.Push(0)
	stack.Push(-3)
	fmt.Printf("Min: %d\n", stack.GetMin()) // -3
	stack.Pop()
	fmt.Printf("Top: %d\n", stack.Top())    // 0
	fmt.Printf("Min: %d\n", stack.GetMin()) // -2
}
