impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut output = vec![1i32; n];

        let mut prefix = 1;
        for i in 0..n {
            output[i] = prefix;
            prefix *= nums[i];
        }

        let mut suffix = 1;
        for i in (0..n).rev() {
            output[i] *= suffix;
            suffix *= nums[i];
        }

        output
    }
}
