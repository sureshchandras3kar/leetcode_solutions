/*
#162 Find Peak Element - Binary Search Approach
Time: O(log n), Space: O(1)

A peak element is an element that is strictly greater than its neighbors.
The array may contain multiple peaks; return index of any one.
*/

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        /*
        Binary search by comparing mid with its right neighbor.
        If nums[mid] < nums[mid+1], peak is on the right.
        Otherwise, peak is on the left or at mid.
        */
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            let mid_idx = mid as usize;

            // If mid is less than its right neighbor, peak is on right
            if nums[mid_idx] < nums[mid_idx + 1] {
                left = mid + 1;
            } else {
                // Peak is on left or at mid
                right = mid;
            }
        }

        left
    }
}
