impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![1i32; n];
        let mut suffix = vec![1i32; n];

        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }

        for i in (0..n - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }

        (0..n).map(|i| prefix[i] * suffix[i]).collect()
    }
}
