/// Dynamic programming approach: forward pass tracking the furthest
/// reachable index. If we can ever reach the end index, return true.
///
/// Time: O(n), Space: O(1)
pub fn can_jump_dp(nums: Vec<i32>) -> bool {
    let mut max_reach = 0;
    for i in 0..nums.len() {
        if i > max_reach {
            return false;
        }
        max_reach = max_reach.max(i + nums[i] as usize);
        if max_reach >= nums.len() - 1 {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", can_jump_dp(vec![2, 3, 1, 1, 4]));  // true
    println!("{}", can_jump_dp(vec![3, 2, 1, 0, 4]));  // false
    println!("{}", can_jump_dp(vec![0]));              // true
    println!("{}", can_jump_dp(vec![2, 0, 0]));        // true
    println!("{}", can_jump_dp(vec![0, 2, 3]));        // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_dp() {
        assert_eq!(can_jump_dp(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump_dp(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(can_jump_dp(vec![0]), true);
        assert_eq!(can_jump_dp(vec![2, 0, 0]), true);
        assert_eq!(can_jump_dp(vec![0, 2, 3]), false);
    }
}
