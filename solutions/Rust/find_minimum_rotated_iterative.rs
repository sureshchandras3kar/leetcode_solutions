pub fn find_min(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            return nums[i + 1];
        }
    }
    nums[0]
}
