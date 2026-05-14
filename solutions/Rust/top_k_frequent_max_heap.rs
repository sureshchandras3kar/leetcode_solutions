use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for num in &nums {
            *freq.entry(*num).or_insert(0) += 1;
        }

        let mut pairs: Vec<(i32, i32)> = freq.into_iter().map(|(num, cnt)| (cnt, num)).collect();
        // Sort by frequency descending
        pairs.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        pairs.into_iter().take(k as usize).map(|(_, num)| num).collect()
    }
}
