use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut best = 0;
        for &n in &num_set {
            if !num_set.contains(&(n - 1)) {  // start of a sequence
                let mut length = 1;
                while num_set.contains(&(n + length)) {
                    length += 1;
                }
                best = best.max(length);
            }
        }
        best
    }
}
