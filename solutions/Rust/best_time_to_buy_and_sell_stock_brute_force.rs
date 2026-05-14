// Approach: Brute Force
// Try every pair (i, j) where i < j. Profit = prices[j] - prices[i].
// Track the maximum profit across all pairs.
//
// Time Complexity: O(n²) — nested loops over all pairs
// Space Complexity: O(1)

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut max_profit_val = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let profit = prices[j] - prices[i];
            max_profit_val = max_profit_val.max(profit);
        }
    }

    max_profit_val
}

fn main() {
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));  // 5
    println!("{}", max_profit(vec![7, 6, 4, 3, 1]));     // 0
    println!("{}", max_profit(vec![2, 4, 1, 7, 5, 11])); // 9
}
