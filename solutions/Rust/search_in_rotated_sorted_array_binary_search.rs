/*
#33 Search in Rotated Sorted Array - Binary Search Approach
Time: O(log n), Space: O(1)

A rotated sorted array has been rotated at an unknown pivot.
Use binary search to find target, identifying which half is sorted.
*/

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        /*
        Binary search on rotated array.
        Determine which half is sorted, then check if target is in that half.
        */
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = (left + (right - left) / 2) as usize;

            if nums[mid] == target {
                return mid as i32;
            }

            // Determine which half is sorted
            if nums[left as usize] <= nums[mid] {
                // Left half is sorted
                if nums[left as usize] <= target && target < nums[mid] {
                    // Target is in sorted left half
                    right = mid as i32 - 1;
                } else {
                    // Target is in right half
                    left = mid as i32 + 1;
                }
            } else {
                // Right half is sorted
                if nums[mid] < target && target <= nums[right as usize] {
                    // Target is in sorted right half
                    left = mid as i32 + 1;
                } else {
                    // Target is in left half
                    right = mid as i32 - 1;
                }
            }
        }

        -1
    }
}
