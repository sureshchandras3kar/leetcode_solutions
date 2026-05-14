/**
 * Find length of longest increasing subsequence using DP O(n²).
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n)
 */
public class LIsDpN2 {
    public static int lisDP(int[] nums) {
        if (nums.length == 0) return 0;

        int[] dp = new int[nums.length];
        for (int i = 0; i < nums.length; i++) {
            dp[i] = 1;
        }

        for (int i = 1; i < nums.length; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[j] < nums[i]) {
                    dp[i] = Math.max(dp[i], dp[j] + 1);
                }
            }
        }

        int max = 0;
        for (int val : dp) {
            max = Math.max(max, val);
        }
        return max;
    }

    public static void main(String[] args) {
        System.out.println(lisDP(new int[]{10, 9, 2, 5, 3, 7, 101, 18}));  // 4
    }
}
