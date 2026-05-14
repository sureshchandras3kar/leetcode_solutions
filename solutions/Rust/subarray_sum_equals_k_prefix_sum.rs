use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        count.insert(0, 1);
        let mut prefix = 0;
        let mut result = 0;
        for num in nums {
            prefix += num;
            result += count.get(&(prefix - k)).copied().unwrap_or(0);
            *count.entry(prefix).or_insert(0) += 1;
        }
        result
    }
}
