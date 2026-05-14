/// Greedy approach: work backwards from the end to find the furthest index
/// we can reach. If we can reach index 0, we can reach the end.
///
/// Time: O(n), Space: O(1)
pub fn can_jump_greedy(nums: Vec<i32>) -> bool {
    let mut last_good_index = nums.len() - 1;
    for i in (0..nums.len() - 1).rev() {
        if i + nums[i] as usize >= last_good_index {
            last_good_index = i;
        }
    }
    last_good_index == 0
}

fn main() {
    println!("{}", can_jump_greedy(vec![2, 3, 1, 1, 4]));  // true
    println!("{}", can_jump_greedy(vec![3, 2, 1, 0, 4]));  // false
    println!("{}", can_jump_greedy(vec![0]));              // true
    println!("{}", can_jump_greedy(vec![2, 0, 0]));        // true
    println!("{}", can_jump_greedy(vec![0, 2, 3]));        // false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_greedy() {
        assert_eq!(can_jump_greedy(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(can_jump_greedy(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(can_jump_greedy(vec![0]), true);
        assert_eq!(can_jump_greedy(vec![2, 0, 0]), true);
        assert_eq!(can_jump_greedy(vec![0, 2, 3]), false);
    }
}
