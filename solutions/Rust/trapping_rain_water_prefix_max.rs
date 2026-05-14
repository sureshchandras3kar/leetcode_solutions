// Approach: Prefix Max Arrays
// Build left_max[i] = max height from height[0] to height[i].
// Build right_max[i] = max height from height[i] to height[n-1].
// Water at i = max(0, min(left_max[i], right_max[i]) - height[i]).
// The minimum of both maxes is the effective wall height that bounds the water.
//
// Time Complexity: O(n) — three linear passes
// Space Complexity: O(n) — two extra arrays of size n

fn trap(height: &[i32]) -> i32 {
    let n = height.len();
    if n == 0 {
        return 0;
    }

    let mut left_max = vec![0i32; n];
    let mut right_max = vec![0i32; n];

    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }

    (0..n)
        .map(|i| 0i32.max(left_max[i].min(right_max[i]) - height[i]))
        .sum()
}

fn main() {
    println!("{}", trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])); // 6
    println!("{}", trap(&[4, 2, 0, 3, 2, 5]));                    // 9
}
