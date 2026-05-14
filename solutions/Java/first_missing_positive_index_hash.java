class Solution {
    public int firstMissingPositive(int[] nums) {
        int n = nums.length;

        // Phase 1: replace non-positives and values > n with n+1
        for (int i = 0; i < n; i++) {
            if (nums[i] <= 0 || nums[i] > n) {
                nums[i] = n + 1;
            }
        }

        // Phase 2: mark presence using negative signs
        for (int i = 0; i < n; i++) {
            int v = Math.abs(nums[i]);
            if (v >= 1 && v <= n) {
                nums[v - 1] = -Math.abs(nums[v - 1]);
            }
        }

        // Phase 3: find first unmarked index
        for (int i = 0; i < n; i++) {
            if (nums[i] > 0) {
                return i + 1;
            }
        }

        return n + 1;
    }
}
