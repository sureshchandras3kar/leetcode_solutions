/**
 * #162 Find Peak Element - Linear Scan Approach
 * Time: O(n), Space: O(1)
 *
 * Iterate through array and find the first element greater than its neighbors.
 */

class Solution {
    findPeakElement(nums) {
        /**
         * Linear scan to find first peak.
         * Compare each element with its neighbors.
         */
        for (let i = 0; i < nums.length; i++) {
            // Check if current element is greater than neighbors
            let isPeak = true;

            if (i > 0 && nums[i] <= nums[i - 1]) {
                isPeak = false;
            }

            if (i < nums.length - 1 && nums[i] <= nums[i + 1]) {
                isPeak = false;
            }

            if (isPeak) {
                return i;
            }
        }

        // If no peak found (shouldn't happen given constraints)
        return 0;
    }
}

// Export for CommonJS
module.exports = Solution;
