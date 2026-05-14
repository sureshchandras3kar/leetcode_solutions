/*
#35 Search Insert Position - Linear Search Approach
Time: O(n), Space: O(1)

Iterate through array to find position or insertion point.
*/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        /*
        Linear search through array.
        Return index when found or where it should be inserted.
        */
        for (i, &num) in nums.iter().enumerate() {
            if num >= target {
                return i as i32;
            }
        }

        // If target larger than all elements
        nums.len() as i32
    }
}
