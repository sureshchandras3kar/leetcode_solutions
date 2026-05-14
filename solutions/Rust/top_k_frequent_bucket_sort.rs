use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, usize> = HashMap::new();
        for num in &nums {
            *freq.entry(*num).or_insert(0) += 1;
        }

        let n = nums.len();
        let mut buckets: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for (num, count) in &freq {
            buckets[*count].push(*num);
        }

        let mut result: Vec<i32> = Vec::new();
        for i in (1..=n).rev() {
            for &num in &buckets[i] {
                result.push(num);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
        result
    }
}
