import java.util.Arrays;

class Solution {
    public int longestConsecutive(int[] nums) {
        if (nums.length == 0) return 0;
        Arrays.sort(nums);
        int best = 1, streak = 1;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] == nums[i - 1]) continue;  // skip duplicate
            if (nums[i] == nums[i - 1] + 1) {
                streak++;
                best = Math.max(best, streak);
            } else {
                streak = 1;
            }
        }
        return best;
    }
}
