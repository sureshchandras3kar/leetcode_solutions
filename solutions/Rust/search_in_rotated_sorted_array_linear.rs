/*
#33 Search in Rotated Sorted Array - Linear Search Approach
Time: O(n), Space: O(1)

Linear scan through array to find target value.
*/

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        /*
        Simple linear search through array.
        Return index if found, otherwise -1.
        */
        for (i, &num) in nums.iter().enumerate() {
            if num == target {
                return i as i32;
            }
        }

        -1
    }
}
