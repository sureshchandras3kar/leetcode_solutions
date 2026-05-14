/*
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Brute Force
Scan the array. When an element equals val, remove it by slicing out that element,
which shifts all subsequent elements left by one position. Do not increment the index
after removal so shifted elements are re-examined.

Time Complexity: O(n²) — worst case when all elements equal val, each removal is O(n)
Space Complexity: O(1) — no extra data structures beyond the input slice

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: First 3 is removed, then second 3 is removed.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: Each 2 is removed by slicing and shifting subsequent elements left.
*/

func removeElement(nums []int, val int) int {
    i := 0
    for i < len(nums) {
        if nums[i] == val {
            // Remove element at index i by slicing: append elements before i with elements after i
            nums = append(nums[:i], nums[i+1:]...)
        } else {
            i++
        }
    }
    return len(nums)
}

/*
Test cases:
nums1 := []int{3, 2, 2, 3}
fmt.Println(removeElement(nums1, 3))  // 2

nums2 := []int{0, 1, 2, 2, 3, 0, 4, 2}
fmt.Println(removeElement(nums2, 2))  // 5

nums3 := []int{2}
fmt.Println(removeElement(nums3, 3))  // 1

nums4 := []int{}
fmt.Println(removeElement(nums4, 0))  // 0
*/
