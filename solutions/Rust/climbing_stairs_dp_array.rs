/**
 * Climb n stairs with dp array approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(n)
 */
pub fn climbing_stairs_dp_array(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

fn main() {
    println!("{}", climbing_stairs_dp_array(3));  // 3
    println!("{}", climbing_stairs_dp_array(4));  // 5
    println!("{}", climbing_stairs_dp_array(5));  // 8
}
