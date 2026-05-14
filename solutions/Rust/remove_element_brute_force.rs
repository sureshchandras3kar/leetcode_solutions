/*
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Brute Force
Scan the array. When an element equals val, remove it using remove() which shifts all
subsequent elements left by one position. Do not increment the index after removal so
shifted elements are re-examined.

Time Complexity: O(n²) — worst case when all elements equal val, each remove is O(n)
Space Complexity: O(1) — no extra data structures beyond the input vector

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: First 3 is removed, then second 3 is removed.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: Each 2 is removed by removing and shifting subsequent elements left.
*/

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
                nums.remove(i);  // Remove element at index i and shift all elements after i one position left
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

/*
Test cases:
let mut nums1 = vec![3, 2, 2, 3];
assert_eq!(Solution::remove_element(&mut nums1, 3), 2);

let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
assert_eq!(Solution::remove_element(&mut nums2, 2), 5);

let mut nums3 = vec![2];
assert_eq!(Solution::remove_element(&mut nums3, 3), 1);

let mut nums4: Vec<i32> = vec![];
assert_eq!(Solution::remove_element(&mut nums4, 0), 0);
*/
