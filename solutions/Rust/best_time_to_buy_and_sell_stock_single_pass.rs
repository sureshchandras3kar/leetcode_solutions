// Approach: Single Pass with Min Price Tracking
// Track the minimum price seen so far and calculate the maximum profit at each price.
// When we see a new price, we check the profit if we sell at that price.
//
// Time Complexity: O(n) — single pass
// Space Complexity: O(1)

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() || prices.len() < 2 {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit_val = 0;

    for i in 1..prices.len() {
        let profit = prices[i] - min_price;
        max_profit_val = max_profit_val.max(profit);
        min_price = min_price.min(prices[i]);
    }

    max_profit_val
}

fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));  // 5
    println!("{}", max_profit(vec![7, 6, 4, 3, 1]));     // 0
    println!("{}", max_profit(vec![2, 4, 1, 7, 5, 11])); // 9
}
