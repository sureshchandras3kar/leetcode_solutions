impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        // Phase 1: replace non-positives and values > n with n+1
        for i in 0..nums.len() {
            if nums[i] <= 0 || nums[i] > n {
                nums[i] = n + 1;
            }
        }

        // Phase 2: mark presence using negative signs
        for i in 0..nums.len() {
            let v = nums[i].abs();
            if v >= 1 && v <= n {
                let idx = (v - 1) as usize;
                let cur = nums[idx].abs();
                nums[idx] = -cur;
            }
        }

        // Phase 3: find first unmarked index
        for i in 0..nums.len() {
            if nums[i] > 0 {
                return (i as i32) + 1;
            }
        }

        n + 1
    }
}
