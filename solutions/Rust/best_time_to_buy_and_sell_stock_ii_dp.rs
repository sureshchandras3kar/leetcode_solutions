fn best_time_to_buy_and_sell_stock_ii_dp(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut hold = -prices[0];  // profit if holding stock after day 0
    let mut no_hold = 0;        // profit if not holding stock after day 0

    for i in 1..prices.len() {
        // Either hold from before or buy today (reset from no_hold)
        hold = hold.max(no_hold - prices[i]);
        // Either don't hold from before or sell today (from hold)
        no_hold = no_hold.max(hold + prices[i]);
    }

    no_hold
}

fn main() {
    let prices1 = vec![7, 1, 5, 3, 6, 4];
    println!("{}", best_time_to_buy_and_sell_stock_ii_dp(prices1));  // 7

    let prices2 = vec![1, 2, 3, 4, 5];
    println!("{}", best_time_to_buy_and_sell_stock_ii_dp(prices2));  // 4

    let prices3 = vec![7, 6, 4, 3, 1];
    println!("{}", best_time_to_buy_and_sell_stock_ii_dp(prices3));  // 0
}
