/**
 * Find minimum number of coins needed to make amount using DP array.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
pub fn coin_change_dp(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in &coins {
            let coin = coin as usize;
            if coin <= i && dp[i - coin] != i32::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] == i32::MAX {
        -1
    } else {
        dp[amount]
    }
}

fn main() {
    println!("{}", coin_change_dp(vec![1, 2, 5], 5));   // 1
    println!("{}", coin_change_dp(vec![2], 3));        // -1
}
