/*
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Brute Force
Scan the array. When an element equals val, shift all subsequent elements left by one position.
This is implemented by copying elements and tracking the new length. Do not increment the
index after removing so shifted elements are re-examined.

Time Complexity: O(n²) — worst case when all elements equal val, each shift is O(n)
Space Complexity: O(1) — no extra data structures beyond the input array

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: First 3 is removed, then second 3 is removed.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: Each 2 is removed by shifting subsequent elements left.
*/

class Solution {
    public int removeElement(int[] nums, int val) {
        int length = nums.length;
        int i = 0;
        while (i < length) {
            if (nums[i] == val) {
                // Shift all elements after i one position left
                for (int j = i; j < length - 1; j++) {
                    nums[j] = nums[j + 1];
                }
                length--;  // Decrease length instead of removing from array
            } else {
                i++;
            }
        }
        return length;
    }
}

/*
Test cases:
Solution sol = new Solution();
int[] nums1 = {3, 2, 2, 3};
System.out.println(sol.removeElement(nums1, 3));  // 2

int[] nums2 = {0, 1, 2, 2, 3, 0, 4, 2};
System.out.println(sol.removeElement(nums2, 2));  // 5

int[] nums3 = {2};
System.out.println(sol.removeElement(nums3, 3));  // 1

int[] nums4 = {};
System.out.println(sol.removeElement(nums4, 0));  // 0
*/
