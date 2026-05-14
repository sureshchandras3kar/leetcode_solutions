package main

import (
	"fmt"
)

/**
 * Min Stack using two stacks (dual stack approach).
 *
 * Maintains a main stack for all values and a separate min stack
 * that tracks the minimum value at each level.
 *
 * Time: O(1) per operation
 * Space: O(n)
 */
type MinStack struct {
	mainStack []int
	minStack  []int
}

func NewMinStack() *MinStack {
	return &MinStack{
		mainStack: []int{},
		minStack:  []int{},
	}
}

func (ms *MinStack) Push(val int) {
	ms.mainStack = append(ms.mainStack, val)
	// Push to minStack the minimum of val and current min
	if len(ms.minStack) == 0 {
		ms.minStack = append(ms.minStack, val)
	} else {
		min := val
		if ms.minStack[len(ms.minStack)-1] < val {
			min = ms.minStack[len(ms.minStack)-1]
		}
		ms.minStack = append(ms.minStack, min)
	}
}

func (ms *MinStack) Pop() {
	ms.mainStack = ms.mainStack[:len(ms.mainStack)-1]
	ms.minStack = ms.minStack[:len(ms.minStack)-1]
}

func (ms *MinStack) Top() int {
	return ms.mainStack[len(ms.mainStack)-1]
}

func (ms *MinStack) GetMin() int {
	return ms.minStack[len(ms.minStack)-1]
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
