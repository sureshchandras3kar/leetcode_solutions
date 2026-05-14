package main

import "fmt"

// Approach: Two Pointers
// Use left and right pointers, tracking maxLeft and maxRight seen so far.
// Process whichever side has the smaller max — that side's max is the bottleneck.
// water at current position = max_on_that_side - height[current]
// Update max before adding water, then advance the pointer.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1) — four variables only

func trap(height []int) int {
    left, right := 0, len(height)-1
    maxLeft, maxRight := 0, 0
    water := 0
    for left < right {
        if maxLeft <= maxRight {
            if height[left] > maxLeft {
                maxLeft = height[left]
            }
            water += maxLeft - height[left]
            left++
        } else {
            if height[right] > maxRight {
                maxRight = height[right]
            }
            water += maxRight - height[right]
            right--
        }
    }
    return water
}

func main() {
    fmt.Println(trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})) // 6
    fmt.Println(trap([]int{4, 2, 0, 3, 2, 5}))                    // 9
}
