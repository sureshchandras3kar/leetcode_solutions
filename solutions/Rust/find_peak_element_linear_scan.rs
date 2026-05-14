/*
#162 Find Peak Element - Linear Scan Approach
Time: O(n), Space: O(1)

Iterate through array and find the first element greater than its neighbors.
*/

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        /*
        Linear scan to find first peak.
        Compare each element with its neighbors.
        */
        for i in 0..nums.len() {
            // Check if current element is greater than neighbors
            let mut is_peak = true;

            if i > 0 && nums[i] <= nums[i - 1] {
                is_peak = false;
            }

            if i < nums.len() - 1 && nums[i] <= nums[i + 1] {
                is_peak = false;
            }

            if is_peak {
                return i as i32;
            }
        }

        // If no peak found (shouldn't happen given constraints)
        0
    }
}
