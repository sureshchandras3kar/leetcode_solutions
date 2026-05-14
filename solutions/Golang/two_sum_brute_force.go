package main

import "fmt"

func twoSumBruteForce(nums []int, target int) []int {
    n := len(nums)
    for i := 0; i < n; i++ {
        for j := i + 1; j < n; j++ {
            if nums[i]+nums[j] == target {
                return []int{i, j}
            }
        }
    }
    return []int{} // Return an empty slice if no valid pair is found
}

func main() {
    nums := []int{2, 7, 11, 15}
    target := 9
    result := twoSumBruteForce(nums, target)
    fmt.Println(result)
}
