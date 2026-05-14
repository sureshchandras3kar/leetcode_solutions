/*
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Brute Force
Scan the array. When an element equals val, remove it using splice() which shifts all
subsequent elements left by one position. Do not increment the index after removal so
shifted elements are re-examined.

Time Complexity: O(n²) — worst case when all elements equal val, each splice is O(n)
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
Explanation: Each 2 is removed by splicing and shifting subsequent elements left.
*/

/**
 * @param {number[]} nums
 * @param {number} val
 * @return {number}
 */
function removeElement(nums, val) {
    let i = 0;
    while (i < nums.length) {
        if (nums[i] === val) {
            nums.splice(i, 1);  // Remove element at index i and shift all elements after i one position left
        } else {
            i++;
        }
    }
    return nums.length;
}

// Test cases
console.log(removeElement([3, 2, 2, 3], 3));  // 2
console.log(removeElement([0, 1, 2, 2, 3, 0, 4, 2], 2));  // 5
console.log(removeElement([2], 3));  // 1
console.log(removeElement([], 0));  // 0
