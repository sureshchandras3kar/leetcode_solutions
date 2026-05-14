package main

import "fmt"

/*
Two-Pointer In-Place Approach
Remove duplicates from sorted array in-place and return the length of the new array.

Time Complexity: O(n)
Space Complexity: O(1)
*/
func removeDuplicates(nums []int) int {
    if len(nums) == 0 {
        return 0
    }

    k := 1  // First element is always unique
    for i := 1; i < len(nums); i++ {
        if nums[i] != nums[i-1] {
            nums[k] = nums[i]
            k++
        }
    }

    return k
}

func main() {
    nums1 := []int{1, 1, 2}
    fmt.Println(removeDuplicates(nums1))  // 2, nums = [1, 2, _]

    nums2 := []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}
    fmt.Println(removeDuplicates(nums2))  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]
}
