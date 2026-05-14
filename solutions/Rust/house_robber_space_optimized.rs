/**
 * Rob houses with maximum money using space-optimized approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
pub fn house_robber_space_optimized(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }

    let mut prev2 = nums[0];
    let mut prev1 = nums[0].max(nums[1]);

    for i in 2..nums.len() {
        let current = (nums[i] + prev2).max(prev1);
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

fn main() {
    println!("{}", house_robber_space_optimized(vec![1, 2, 3, 1]));      // 4
    println!("{}", house_robber_space_optimized(vec![2, 7, 9, 3, 1]));   // 12
}
