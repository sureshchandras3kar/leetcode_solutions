impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            let mut total = 0;
            for j in i..nums.len() {
                total += nums[j];
                if total == k {
                    result += 1;
                }
            }
        }
        result
    }
}
