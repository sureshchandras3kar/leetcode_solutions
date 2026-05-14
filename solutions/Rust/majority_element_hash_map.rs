use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let threshold = nums.len() / 2;
        for num in nums {
            let next = counts.entry(num).or_insert(0);
            *next += 1;
            if *next > threshold as i32 {
                return num;
            }
        }
        0
    }
}
