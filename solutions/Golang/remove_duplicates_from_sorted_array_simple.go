package main

import "fmt"

/*
Simple Pass Approach
Remove duplicates by iterating and comparing consecutive elements.

Time Complexity: O(n)
Space Complexity: O(1)
*/
func removeDuplicates(nums []int) int {
    if len(nums) == 0 {
        return 0
    }

    writeIdx := 0
    for readIdx := 1; readIdx < len(nums); readIdx++ {
        if nums[readIdx] != nums[writeIdx] {
            writeIdx++
            nums[writeIdx] = nums[readIdx]
        }
    }

    return writeIdx + 1
}

func main() {
    nums1 := []int{1, 1, 2}
    fmt.Println(removeDuplicates(nums1))  // 2, nums = [1, 2, _]

    nums2 := []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}
    fmt.Println(removeDuplicates(nums2))  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
}
