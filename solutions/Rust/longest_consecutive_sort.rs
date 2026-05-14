impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.sort_unstable();
        let mut best = 1;
        let mut streak = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                continue;  // skip duplicate
            }
            if nums[i] == nums[i - 1] + 1 {
                streak += 1;
                best = best.max(streak);
            } else {
                streak = 1;
            }
        }
        best
    }
}
