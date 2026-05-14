class Solution {
    public int firstMissingPositive(int[] nums) {
        int n = nums.length;
        int i = 0;

        // Place each number at its correct index (value v at index v-1)
        while (i < n) {
            int j = nums[i] - 1;
            if (nums[i] >= 1 && nums[i] <= n && nums[j] != nums[i]) {
                int temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
            } else {
                i++;
            }
        }

        // Find first index where value doesn't match expected
        for (int k = 0; k < n; k++) {
            if (nums[k] != k + 1) {
                return k + 1;
            }
        }

        return n + 1;
    }
}
