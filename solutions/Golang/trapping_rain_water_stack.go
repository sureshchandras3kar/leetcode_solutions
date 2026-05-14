package main

import "fmt"

// Approach: Monotonic Stack
// Maintain a stack of indices with decreasing heights (a monotonic decreasing stack).
// When height[i] > height[stack top], we found a right wall — the pool can be filled.
// Pop the bottom index, compute water between the new stack top (left wall) and i (right wall).
// Think horizontally: each pop fills one horizontal layer of trapped water.
//
// Time Complexity: O(n) — each bar is pushed and popped at most once
// Space Complexity: O(n) — stack stores indices

func trap(height []int) int {
    stack := []int{}
    water := 0
    for i := 0; i < len(height); i++ {
        for len(stack) > 0 && height[i] > height[stack[len(stack)-1]] {
            bottom := stack[len(stack)-1]
            stack = stack[:len(stack)-1]
            if len(stack) == 0 {
                break
            }
            left := stack[len(stack)-1]
            width := i - left - 1
            boundedHeight := height[i]
            if height[left] < boundedHeight {
                boundedHeight = height[left]
            }
            boundedHeight -= height[bottom]
            water += boundedHeight * width
        }
        stack = append(stack, i)
    }
    return water
}

func main() {
    fmt.Println(trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})) // 6
    fmt.Println(trap([]int{4, 2, 0, 3, 2, 5}))                    // 9
}
