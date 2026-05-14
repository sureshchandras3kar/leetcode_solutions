impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;

        // Place each number at its correct index (value v at index v-1)
        while i < n {
            let j = (nums[i] - 1) as usize;
            if nums[i] >= 1 && nums[i] <= n as i32 && nums[j] != nums[i] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        }

        // Find first index where value doesn't match expected
        for i in 0..n {
            if nums[i] != (i as i32) + 1 {
                return (i as i32) + 1;
            }
        }

        (n as i32) + 1
    }
}
