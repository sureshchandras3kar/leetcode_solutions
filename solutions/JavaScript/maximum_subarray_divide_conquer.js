/**
 * Divide and Conquer approach: Split the array, find max subarrays in each half,
 * and at the crossing point, then return the maximum.
 * Time: O(n log n), Space: O(log n) for recursion stack
 */
function maximumSubarrayDivideConquer(nums) {
    function maxCrossingSubarray(nums, left, mid, right) {
        // Sum from mid to left
        let left_sum = Number.NEGATIVE_INFINITY;
        let sum = 0;
        for (let i = mid; i >= left; i--) {
            sum += nums[i];
            left_sum = Math.max(left_sum, sum);
        }

        // Sum from mid+1 to right
        let right_sum = Number.NEGATIVE_INFINITY;
        sum = 0;
        for (let i = mid + 1; i <= right; i++) {
            sum += nums[i];
            right_sum = Math.max(right_sum, sum);
        }

        return left_sum + right_sum;
    }

    function helper(nums, left, right) {
        if (left === right) {
            return nums[left];
        }

        const mid = Math.floor((left + right) / 2);

        // Maximum in left half, right half, and crossing the middle
        const left_max = helper(nums, left, mid);
        const right_max = helper(nums, mid + 1, right);
        const cross_max = maxCrossingSubarray(nums, left, mid, right);

        return Math.max(left_max, right_max, cross_max);
    }

    return helper(nums, 0, nums.length - 1);
}

console.log(maximumSubarrayDivideConquer([-2, 1, -3, 4, -1, 2, 1, -5, 4]));  // 6
console.log(maximumSubarrayDivideConquer([5]));  // 5
console.log(maximumSubarrayDivideConquer([-5, -2, -3]));  // -2
