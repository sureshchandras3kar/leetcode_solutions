public class MaxSubArray_Kadane {
    /**
     * Kadane's Algorithm - Find the maximum sum of any contiguous subarray.
     *
     * Time Complexity: O(n) - Single pass through the array
     * Space Complexity: O(1) - Only tracking current and max sums
     *
     * @param nums Array of integers
     * @return Maximum subarray sum
     */
    public static int maxSubArray(int[] nums) {
        int maxSum = nums[0];
        int currentSum = nums[0];

        for (int i = 1; i < nums.length; i++) {
            // Decide whether to extend the existing subarray or start fresh
            currentSum = Math.max(nums[i], currentSum + nums[i]);
            // Update overall maximum
            maxSum = Math.max(maxSum, currentSum);
        }

        return maxSum;
    }

    public static void main(String[] args) {
        int[] test1 = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
        System.out.println("Test 1: " + maxSubArray(test1)); // Expected: 6
    }
}
