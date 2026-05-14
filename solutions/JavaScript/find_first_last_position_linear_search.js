/**
 * #34 Find First and Last Position of Element in Sorted Array - Linear Search
 * Time: O(n), Space: O(1)
 *
 * Linear scan to find first and last occurrences of target value.
 */

class Solution {
    searchRange(nums, target) {
        /**
         * Linear search to find first and last positions.
         */
        if (nums.length === 0) {
            return [-1, -1];
        }

        let firstPos = -1;
        let lastPos = -1;

        // Find first position
        for (let i = 0; i < nums.length; i++) {
            if (nums[i] === target) {
                firstPos = i;
                break;
            }
        }

        // If target not found
        if (firstPos === -1) {
            return [-1, -1];
        }

        // Find last position
        for (let i = nums.length - 1; i >= 0; i--) {
            if (nums[i] === target) {
                lastPos = i;
                break;
            }
        }

        return [firstPos, lastPos];
    }
}

// Export for CommonJS
module.exports = Solution;
