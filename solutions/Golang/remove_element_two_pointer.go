/*
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Two Pointers
Use a 'write' pointer to place non-val elements and a 'read' pointer to scan the array.
When read finds a non-val element, copy it to the write position and advance both.
When read finds a val element, skip it and only advance read.

Time Complexity: O(n) — single pass through the array
Space Complexity: O(1) — no extra data structures

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: The first 2 elements are [2, 2]; elements after position 2 are ignored.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: The first 5 elements contain no 2.
*/

func removeElement(nums []int, val int) int {
    write := 0
    for read := 0; read < len(nums); read++ {
        if nums[read] != val {
            nums[write] = nums[read]
            write++
        }
    }
    return write
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
