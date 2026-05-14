/**
 * Divide and Conquer approach: Split the array, find max subarrays in each half,
 * and at the crossing point, then return the maximum.
 * Time: O(n log n), Space: O(log n) for recursion stack
 */
public class MaximumSubarrayDivideConquer {
    private int maxCrossingSubarray(int[] nums, int left, int mid, int right) {
        // Sum from mid to left
        int left_sum = Integer.MIN_VALUE;
        int sum = 0;
        for (int i = mid; i >= left; i--) {
            sum += nums[i];
            left_sum = Math.max(left_sum, sum);
        }

        // Sum from mid+1 to right
        int right_sum = Integer.MIN_VALUE;
        sum = 0;
        for (int i = mid + 1; i <= right; i++) {
            sum += nums[i];
            right_sum = Math.max(right_sum, sum);
        }

        return left_sum + right_sum;
    }

    private int helper(int[] nums, int left, int right) {
        if (left == right) {
            return nums[left];
        }

        int mid = left + (right - left) / 2;

        // Maximum in left half, right half, and crossing the middle
        int left_max = helper(nums, left, mid);
        int right_max = helper(nums, mid + 1, right);
        int cross_max = maxCrossingSubarray(nums, left, mid, right);

        return Math.max(left_max, Math.max(right_max, cross_max));
    }

    public int maxSubarray(int[] nums) {
        return helper(nums, 0, nums.length - 1);
    }

    public static void main(String[] args) {
        MaximumSubarrayDivideConquer solution = new MaximumSubarrayDivideConquer();

        System.out.println(solution.maxSubarray(new int[]{-2, 1, -3, 4, -1, 2, 1, -5, 4}));  // 6
        System.out.println(solution.maxSubarray(new int[]{5}));  // 5
        System.out.println(solution.maxSubarray(new int[]{-5, -2, -3}));  // -2
    }
}
