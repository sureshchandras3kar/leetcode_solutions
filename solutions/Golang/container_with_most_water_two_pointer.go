package main

import "fmt"

// Approach: Two Pointers
// Place one pointer at the start and one at the end.
// Area = min(height[left], height[right]) * (right - left).
// Move the pointer with the shorter height inward — the shorter wall is the bottleneck.
//
// Time Complexity: O(n)
// Space Complexity: O(1)

func maxArea(height []int) int {
    left := 0
    right := len(height) - 1
    maxWater := 0
    for left < right {
        h := height[left]
        if height[right] < h {
            h = height[right]
        }
        water := h * (right - left)
        if water > maxWater {
            maxWater = water
        }
        if height[left] <= height[right] {
            left++
        } else {
            right--
        }
    }
    return maxWater
}

func main() {
    fmt.Println(maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7})) // 49
    fmt.Println(maxArea([]int{1, 1}))                        // 1
}
