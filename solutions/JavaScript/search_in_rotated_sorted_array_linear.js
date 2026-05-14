/**
 * #33 Search in Rotated Sorted Array - Linear Search Approach
 * Time: O(n), Space: O(1)
 *
 * Linear scan through array to find target value.
 */

class Solution {
    search(nums, target) {
        /**
         * Simple linear search through array.
         * Return index if found, otherwise -1.
         */
        for (let i = 0; i < nums.length; i++) {
            if (nums[i] === target) {
                return i;
            }
        }

        return -1;
    }
}

// Export for CommonJS
module.exports = Solution;
