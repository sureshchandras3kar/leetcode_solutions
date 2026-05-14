// Approach: Brute Force
// Try every pair (i, j) where i < j. Profit = prices[j] - prices[i].
// Track the maximum profit across all pairs.
//
// Time Complexity: O(n²) — nested loops over all pairs
// Space Complexity: O(1)

class Solution {
    public int maxProfit(int[] prices) {
        int n = prices.length;
        int maxProfitVal = 0;

        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                int profit = prices[j] - prices[i];
                maxProfitVal = Math.max(maxProfitVal, profit);
            }
        }

        return maxProfitVal;
    }

    public static void main(String[] args) {
        Solution sol = new Solution();

        int[] prices1 = {7, 1, 5, 3, 6, 4};
        System.out.println(sol.maxProfit(prices1));  // 5

        int[] prices2 = {7, 6, 4, 3, 1};
        System.out.println(sol.maxProfit(prices2));  // 0

        int[] prices3 = {2, 4, 1, 7, 5, 11};
        System.out.println(sol.maxProfit(prices3));  // 9
    }
}
