public class Main {
    public static int bestTimeToBuyAndSellStockIIDP(int[] prices) {
        if (prices == null || prices.length < 2) {
            return 0;
        }

        int hold = -prices[0];      // profit if holding stock after day 0
        int noHold = 0;             // profit if not holding stock after day 0

        for (int i = 1; i < prices.length; i++) {
            // Either hold from before or buy today (reset from noHold)
            hold = Math.max(hold, noHold - prices[i]);
            // Either don't hold from before or sell today (from hold)
            noHold = Math.max(noHold, hold + prices[i]);
        }

        return noHold;
    }

    public static void main(String[] args) {
        int[] prices1 = {7, 1, 5, 3, 6, 4};
        System.out.println(bestTimeToBuyAndSellStockIIDP(prices1));  // 7

        int[] prices2 = {1, 2, 3, 4, 5};
        System.out.println(bestTimeToBuyAndSellStockIIDP(prices2));  // 4

        int[] prices3 = {7, 6, 4, 3, 1};
        System.out.println(bestTimeToBuyAndSellStockIIDP(prices3));  // 0
    }
}
