package main

import (
    "fmt"
    "math"
)

// Dynamic Programming approach: compute minimum jumps needed to reach each index.
//
// Intuition: dp[i] = minimum number of jumps to reach index i.
// For each index i, look back at all indices j where j + nums[j] >= i,
// meaning from j we can reach i in one jump. Take the minimum jumps[j] + 1.
//
// Time Complexity: O(n²) - for each index, we may check all previous indices
// Space Complexity: O(n) - dp array
func jumpGameIIDP(nums []int) int {
    if len(nums) <= 1 {
        return 0
    }

    n := len(nums)
    dp := make([]int, n)
    for i := 0; i < n; i++ {
        dp[i] = int(math.MaxInt32)
    }
    dp[0] = 0  // Start at index 0 with 0 jumps

    for i := 1; i < n; i++ {
        // Check all previous indices to see which can reach i
        for j := 0; j < i; j++ {
            if j+nums[j] >= i {  // From index j, we can reach index i
                if dp[j]+1 < dp[i] {
                    dp[i] = dp[j] + 1
                }
                break  // Optimization: once we find a j that reaches i, we can stop
            }
        }
    }

    return dp[n-1]
}

func main() {
    fmt.Println(jumpGameIIDP([]int{2, 3, 1, 1, 4}))        // 2
    fmt.Println(jumpGameIIDP([]int{2, 3, 0, 6, 9, 1, 2}))  // 2
    fmt.Println(jumpGameIIDP([]int{10}))                   // 0
    fmt.Println(jumpGameIIDP([]int{1, 1, 1, 0}))           // 3
}
