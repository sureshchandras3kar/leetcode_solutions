/*
#35 Search Insert Position - Binary Search Approach
Time: O(log n), Space: O(1)

Given a sorted array and a target value, return the index if found,
otherwise return the index where it would be if it were inserted.
*/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        /*
        Binary search to find insert position.
        Left pointer converges to insertion point.
        */
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];

            if mid_val == target {
                return mid;
            } else if mid_val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // left is at insertion position when target not found
        left
    }
}
