/*
#34 Find First and Last Position of Element in Sorted Array - Binary Search
Time: O(log n), Space: O(1)

Find the starting and ending position of a given target value in a sorted array.
Use two binary searches: one to find first position, one to find last.
*/

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        /*
        Two binary searches to find first and last positions.
        */
        if nums.is_empty() {
            return vec![-1, -1];
        }

        // Find first position
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;
        let mut first_pos = -1;

        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            if nums[mid] == target {
                first_pos = mid as i32;
                right = mid as i32 - 1;  // Continue searching left
            } else if nums[mid] < target {
                left = mid as i32 + 1;
            } else {
                right = mid as i32 - 1;
            }
        }

        // If target not found
        if first_pos == -1 {
            return vec![-1, -1];
        }

        // Find last position
        left = 0;
        right = nums.len() as i32 - 1;
        let mut last_pos = -1;

        while left <= right {
            let mid = (left + (right - left) / 2) as usize;
            if nums[mid] == target {
                last_pos = mid as i32;
                left = mid as i32 + 1;  // Continue searching right
            } else if nums[mid] < target {
                left = mid as i32 + 1;
            } else {
                right = mid as i32 - 1;
            }
        }

        vec![first_pos, last_pos]
    }
}
