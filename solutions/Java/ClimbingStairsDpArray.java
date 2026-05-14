/**
 * Climb n stairs with dp array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
public class ClimbingStairsDpArray {
    public static int climbingStairsDpArray(int n) {
        if (n <= 1) return 1;

        int[] dp = new int[n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for (int i = 2; i <= n; i++) {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        return dp[n];
    }

    public static void main(String[] args) {
        System.out.println(climbingStairsDpArray(3));  // 3
        System.out.println(climbingStairsDpArray(4));  // 5
        System.out.println(climbingStairsDpArray(5));  // 8
    }
}
