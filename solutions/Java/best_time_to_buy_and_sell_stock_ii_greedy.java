public class Main {
    public static int bestTimeToBuyAndSellStockIIGreedy(int[] prices) {
        int maxProfit = 0;
        for (int i = 1; i < prices.length; i++) {
            if (prices[i] > prices[i - 1]) {
                maxProfit += prices[i] - prices[i - 1];
            }
        }
        return maxProfit;
    }

    public static void main(String[] args) {
        int[] prices1 = {7, 1, 5, 3, 6, 4};
        System.out.println(bestTimeToBuyAndSellStockIIGreedy(prices1));  // 7

        int[] prices2 = {1, 2, 3, 4, 5};
        System.out.println(bestTimeToBuyAndSellStockIIGreedy(prices2));  // 4

        int[] prices3 = {7, 6, 4, 3, 1};
        System.out.println(bestTimeToBuyAndSellStockIIGreedy(prices3));  // 0
    }
}
