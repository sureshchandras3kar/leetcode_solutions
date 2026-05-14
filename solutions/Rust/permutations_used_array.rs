pub fn permutations_used_array(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut used = vec![false; nums.len()];
    let mut current = vec![];
    backtrack(&nums, &mut used, &mut current, &mut result);
    result
}

fn backtrack(nums: &Vec<i32>, used: &mut Vec<bool>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    // Base case: we've used all numbers
    if current.len() == nums.len() {
        result.push(current.clone());
        return;
    }

    for i in 0..nums.len() {
        if used[i] {
            continue;
        }

        // Choose
        current.push(nums[i]);
        used[i] = true;
        // Explore
        backtrack(nums, used, current, result);
        // Unchoose
        current.pop();
        used[i] = false;
    }
}

fn main() {
    let result = permutations_used_array(vec![1, 2, 3]);
    println!("{:?}", result);
    // Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
}
