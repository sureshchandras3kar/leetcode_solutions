package main

import "fmt"

// Approach: Brute Force
// Try every pair (i, j). Area = min(height[i], height[j]) * (j - i). Track the maximum.
//
// Time Complexity: O(n^2)
// Space Complexity: O(1)

func maxAreaBruteForce(height []int) int {
    n := len(height)
    maxWater := 0
    for i := 0; i < n; i++ {
        for j := i + 1; j < n; j++ {
            h := height[i]
            if height[j] < h {
                h = height[j]
            }
            water := h * (j - i)
            if water > maxWater {
                maxWater = water
            }
        }
    }
    return maxWater
}

func main() {
    fmt.Println(maxAreaBruteForce([]int{1, 8, 6, 2, 5, 4, 8, 3, 7})) // 49
    fmt.Println(maxAreaBruteForce([]int{1, 1}))                        // 1
}
