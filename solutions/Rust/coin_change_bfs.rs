use std::collections::{VecDeque, HashSet};

/**
 * Find minimum number of coins needed to make amount using BFS.
 *
 * Time Complexity: O(n * amount)
 * Space Complexity: O(amount)
 */
pub fn coin_change_bfs(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((amount, 0));
    visited.insert(amount);

    while let Some((curr_amount, num_coins)) = queue.pop_front() {
        for &coin in &coins {
            let next_amount = curr_amount - coin;
            if next_amount == 0 {
                return num_coins + 1;
            }
            if next_amount > 0 && !visited.contains(&next_amount) {
                visited.insert(next_amount);
                queue.push_back((next_amount, num_coins + 1));
            }
        }
    }

    -1
}

fn main() {
    println!("{}", coin_change_bfs(vec![1, 2, 5], 5));   // 1
    println!("{}", coin_change_bfs(vec![2], 3));        // -1
}
