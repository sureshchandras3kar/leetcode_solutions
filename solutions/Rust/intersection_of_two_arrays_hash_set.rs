use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let seen: HashSet<i32> = nums1.into_iter().collect();
        let mut result = HashSet::new();
        for num in nums2 {
            if seen.contains(&num) {
                result.insert(num);
            }
        }
        result.into_iter().collect()
    }
}
