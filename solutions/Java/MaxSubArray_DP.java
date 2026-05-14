public class MaxSubArray_DP {
    /**
     * Dynamic Programming approach - Build up subarray sums.
     *
     * Time Complexity: O(n)
     * Space Complexity: O(1) - Optimized space version
     *
     * @param nums Array of integers
     * @return Maximum subarray sum
     */
    public static int maxSubArray(int[] nums) {
        int n = nums.length;
        if (n == 1) return nums[0];

        int maxSum = nums[0];
        int dp = nums[0];  // max sum ending at current position

        for (int i = 1; i < n; i++) {
            dp = Math.max(nums[i], dp + nums[i]);
            maxSum = Math.max(maxSum, dp);
        }

        return maxSum;
    }

    public static void main(String[] args) {
        int[] test1 = {-2, 1, -3, 4, -1, 2, 1, -5, 4};
        System.out.println("Test 1: " + maxSubArray(test1)); // Expected: 6
    }
}
