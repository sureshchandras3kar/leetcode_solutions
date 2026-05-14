package main

import "fmt"

// Approach: Prefix Max Arrays
// Build leftMax[i] = max height from height[0] to height[i].
// Build rightMax[i] = max height from height[i] to height[n-1].
// Water at i = max(0, min(leftMax[i], rightMax[i]) - height[i]).
// The minimum of both maxes is the effective wall height that bounds the water.
//
// Time Complexity: O(n) — three linear passes
// Space Complexity: O(n) — two extra arrays of size n

func trap(height []int) int {
    n := len(height)
    if n == 0 {
        return 0
    }

    leftMax := make([]int, n)
    rightMax := make([]int, n)

    leftMax[0] = height[0]
    for i := 1; i < n; i++ {
        if height[i] > leftMax[i-1] {
            leftMax[i] = height[i]
        } else {
            leftMax[i] = leftMax[i-1]
        }
    }

    rightMax[n-1] = height[n-1]
    for i := n - 2; i >= 0; i-- {
        if height[i] > rightMax[i+1] {
            rightMax[i] = height[i]
        } else {
            rightMax[i] = rightMax[i+1]
        }
    }

    water := 0
    for i := 0; i < n; i++ {
        minWall := leftMax[i]
        if rightMax[i] < minWall {
            minWall = rightMax[i]
        }
        if minWall > height[i] {
            water += minWall - height[i]
        }
    }
    return water
}

func main() {
    fmt.Println(trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})) // 6
    fmt.Println(trap([]int{4, 2, 0, 3, 2, 5}))                    // 9
}
