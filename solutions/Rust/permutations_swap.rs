pub fn permutations_swap(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut nums = nums;
    backtrack(&mut nums, 0, &mut result);
    result
}

fn backtrack(nums: &mut Vec<i32>, first: usize, result: &mut Vec<Vec<i32>>) {
    // Base case: all elements are placed
    if first == nums.len() {
        result.push(nums.clone());
        return;
    }

    for i in first..nums.len() {
        // Swap
        nums.swap(first, i);
        // Backtrack
        backtrack(nums, first + 1, result);
        // Swap back
        nums.swap(first, i);
    }
}

fn main() {
    let result = permutations_swap(vec![1, 2, 3]);
    println!("{:?}", result);
    // Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
}
