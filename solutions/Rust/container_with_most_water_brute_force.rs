// Approach: Brute Force
// Try every pair (i, j). Area = min(height[i], height[j]) * (j - i). Track the maximum.
//
// Time Complexity: O(n^2)
// Space Complexity: O(1)

fn max_area_brute_force(height: &[i32]) -> i32 {
    let n = height.len();
    let mut max_water = 0i32;
    for i in 0..n {
        for j in (i + 1)..n {
            let water = height[i].min(height[j]) * (j - i) as i32;
            max_water = max_water.max(water);
        }
    }
    max_water
}

fn main() {
    println!("{}", max_area_brute_force(&[1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
    println!("{}", max_area_brute_force(&[1, 1]));                        // 1
}
