package main

import "fmt"

// Greedy approach: track the farthest reachable index and jump count.
//
// Intuition: We maintain the range [currentEnd] that can be reached
// with the current number of jumps. When we exhaust this range, we increment jumps
// and expand to [currentEnd + 1, farthest], which is reachable with one more jump.
//
// Time Complexity: O(n) - single pass through array
// Space Complexity: O(1) - only use constant extra space
func jumpGameIIGreedy(nums []int) int {
    if len(nums) <= 1 {
        return 0
    }

    jumps := 0
    currentEnd := 0  // End of range reachable with current number of jumps
    farthest := 0    // Farthest index reachable so far

    for i := 0; i < len(nums)-1; i++ {
        // Update the farthest index we can reach
        if i+nums[i] > farthest {
            farthest = i + nums[i]
        }

        // If we've reached the end of current jump range, we must jump
        if i == currentEnd {
            jumps++
            currentEnd = farthest

            // Optimization: if we can reach the end, no need to continue
            if currentEnd >= len(nums)-1 {
                break
            }
        }
    }

    return jumps
}

func main() {
    fmt.Println(jumpGameIIGreedy([]int{2, 3, 1, 1, 4}))        // 2
    fmt.Println(jumpGameIIGreedy([]int{2, 3, 0, 6, 9, 1, 2}))  // 2
    fmt.Println(jumpGameIIGreedy([]int{10}))                   // 0
    fmt.Println(jumpGameIIGreedy([]int{1, 1, 1, 0}))           // 3
}
