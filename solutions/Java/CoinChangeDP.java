import java.util.*;

/**
 * Find minimum number of coins needed to make amount using DP array.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
public class CoinChangeDP {
    public static int coinChangeDP(int[] coins, int amount) {
        int[] dp = new int[amount + 1];
        for (int i = 1; i <= amount; i++) {
            dp[i] = Integer.MAX_VALUE;
        }
        dp[0] = 0;

        for (int i = 1; i <= amount; i++) {
            for (int coin : coins) {
                if (coin <= i && dp[i - coin] != Integer.MAX_VALUE) {
                    dp[i] = Math.min(dp[i], dp[i - coin] + 1);
                }
            }
        }

        return dp[amount] == Integer.MAX_VALUE ? -1 : dp[amount];
    }

    public static void main(String[] args) {
        System.out.println(coinChangeDP(new int[]{1, 2, 5}, 5));   // 1
        System.out.println(coinChangeDP(new int[]{2}, 3));         // -1
    }
}
