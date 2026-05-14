/**
 * Climb n stairs with Fibonacci variables approach (space-optimized).
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
public class ClimbingStairsFibonacciVariables {
    public static int climbingStairsFibonacciVariables(int n) {
        if (n <= 1) return 1;

        int prev2 = 1;  // dp[0]
        int prev1 = 1;  // dp[1]

        for (int i = 2; i <= n; i++) {
            int current = prev1 + prev2;
            prev2 = prev1;
            prev1 = current;
        }

        return prev1;
    }

    public static void main(String[] args) {
        System.out.println(climbingStairsFibonacciVariables(3));  // 3
        System.out.println(climbingStairsFibonacciVariables(4));  // 5
        System.out.println(climbingStairsFibonacciVariables(5));  // 8
    }
}
