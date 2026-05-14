fn best_time_to_buy_and_sell_stock_ii_greedy(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            max_profit += prices[i] - prices[i - 1];
        }
    }
    max_profit
}

fn main() {
    let prices1 = vec![7, 1, 5, 3, 6, 4];
    println!("{}", best_time_to_buy_and_sell_stock_ii_greedy(prices1));  // 7

    let prices2 = vec![1, 2, 3, 4, 5];
    println!("{}", best_time_to_buy_and_sell_stock_ii_greedy(prices2));  // 4

    let prices3 = vec![7, 6, 4, 3, 1];
    println!("{}", best_time_to_buy_and_sell_stock_ii_greedy(prices3));  // 0
}
