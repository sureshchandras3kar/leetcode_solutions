use std::collections::HashSet;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let seen: HashSet<i32> = nums.iter().copied().collect();
        for value in 0..=(nums.len() as i32) {
            if !seen.contains(&value) {
                return value;
            }
        }
        -1
    }
}
