// Approach: Two Pointers
// Place one pointer at the start and one at the end.
// Area = min(height[left], height[right]) * (right - left).
// Move the pointer with the shorter height inward — the shorter wall is the bottleneck.
//
// Time Complexity: O(n)
// Space Complexity: O(1)

fn max_area(height: &[i32]) -> i32 {
    let mut left = 0usize;
    let mut right = height.len() - 1;
    let mut max_water = 0i32;
    while left < right {
        let water = height[left].min(height[right]) * (right - left) as i32;
        max_water = max_water.max(water);
        if height[left] <= height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_water
}

fn main() {
    println!("{}", max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
    println!("{}", max_area(&[1, 1]));                        // 1
}
