// Approach: Single Pass with Min Price Tracking
// Track the minimum price seen so far and calculate the maximum profit at each price.
// When we see a new price, we check the profit if we sell at that price.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1)

class Solution {
    public int maxProfit(int[] prices) {
        if (prices == null || prices.length < 2) {
            return 0;
        }

        int minPrice = prices[0];
        int maxProfitVal = 0;

        for (int i = 1; i < prices.length; i++) {
            int profit = prices[i] - minPrice;
            maxProfitVal = Math.max(maxProfitVal, profit);
            minPrice = Math.min(minPrice, prices[i]);
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
