/**
 * Climb n stairs with Fibonacci variables approach (space-optimized).
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
pub fn climbing_stairs_fibonacci_variables(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    let mut prev2 = 1;  // dp[0]
    let mut prev1 = 1;  // dp[1]

    for _ in 2..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

fn main() {
    println!("{}", climbing_stairs_fibonacci_variables(3));  // 3
    println!("{}", climbing_stairs_fibonacci_variables(4));  // 5
    println!("{}", climbing_stairs_fibonacci_variables(5));  // 8
}
