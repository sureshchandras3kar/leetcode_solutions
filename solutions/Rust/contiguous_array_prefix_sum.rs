use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        seen.insert(0, -1);
        let mut prefix = 0;
        let mut best = 0;
        for (i, &num) in nums.iter().enumerate() {
            prefix += if num == 1 { 1 } else { -1 };
            if let Some(&first) = seen.get(&prefix) {
                best = best.max(i as i32 - first);
            } else {
                seen.insert(prefix, i as i32);
            }
        }
        best
    }
}
