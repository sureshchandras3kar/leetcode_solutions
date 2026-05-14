/// Greedy approach: track the farthest reachable index and jump count.
///
/// Intuition: We maintain the range [current_end] that can be reached
/// with the current number of jumps. When we exhaust this range, we increment jumps
/// and expand to [current_end + 1, farthest], which is reachable with one more jump.
///
/// Time Complexity: O(n) - single pass through array
/// Space Complexity: O(1) - only use constant extra space
pub fn jump_game_ii_greedy(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }

    let mut jumps = 0;
    let mut current_end = 0;  // End of range reachable with current number of jumps
    let mut farthest = 0;      // Farthest index reachable so far

    for i in 0..nums.len() - 1 {
        // Update the farthest index we can reach
        farthest = std::cmp::max(farthest, i as i32 + nums[i]) as usize;

        // If we've reached the end of current jump range, we must jump
        if i == current_end {
            jumps += 1;
            current_end = farthest;

            // Optimization: if we can reach the end, no need to continue
            if current_end >= nums.len() - 1 {
                break;
            }
        }
    }

    jumps
}

fn main() {
    println!("{}", jump_game_ii_greedy(vec![2, 3, 1, 1, 4]));        // 2
    println!("{}", jump_game_ii_greedy(vec![2, 3, 0, 6, 9, 1, 2]));  // 2
    println!("{}", jump_game_ii_greedy(vec![10]));                   // 0
    println!("{}", jump_game_ii_greedy(vec![1, 1, 1, 0]));           // 3
}
