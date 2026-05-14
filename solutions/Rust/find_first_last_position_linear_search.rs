/*
#34 Find First and Last Position of Element in Sorted Array - Linear Search
Time: O(n), Space: O(1)

Linear scan to find first and last occurrences of target value.
*/

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        Linear search to find first and last positions.
        */
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut first_pos = -1;
        let mut last_pos = -1;

        // Find first position
        for (i, &num) in nums.iter().enumerate() {
            if num == target {
                first_pos = i as i32;
                break;
            }
        }

        // If target not found
        if first_pos == -1 {
            return vec![-1, -1];
        }

        // Find last position
        for (i, &num) in nums.iter().enumerate().rev() {
            if num == target {
                last_pos = i as i32;
                break;
            }
        }

        vec![first_pos, last_pos]
    }
}
