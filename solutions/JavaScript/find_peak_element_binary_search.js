/**
 * #162 Find Peak Element - Binary Search Approach
 * Time: O(log n), Space: O(1)
 *
 * A peak element is an element that is strictly greater than its neighbors.
 * The array may contain multiple peaks; return index of any one.
 */

class Solution {
    findPeakElement(nums) {
        /**
         * Binary search by comparing mid with its right neighbor.
         * If nums[mid] < nums[mid+1], peak is on the right.
         * Otherwise, peak is on the left or at mid.
         */
        let left = 0, right = nums.length - 1;

        while (left < right) {
            const mid = left + Math.floor((right - left) / 2);

            // If mid is less than its right neighbor, peak is on right
            if (nums[mid] < nums[mid + 1]) {
                left = mid + 1;
            } else {
                // Peak is on left or at mid
                right = mid;
            }
        }

        return left;
    }
}

// Export for CommonJS
module.exports = Solution;
