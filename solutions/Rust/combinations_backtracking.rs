pub fn combinations_backtracking(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut current = vec![];
    backtrack(n, k, 1, &mut current, &mut result);
    result
}

fn backtrack(n: i32, k: i32, start: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    // Base case: we've selected k numbers
    if current.len() == k as usize {
        result.push(current.clone());
        return;
    }

    // Explore: try each remaining number
    for i in start..=n {
        // Choose
        current.push(i);
        // Explore
        backtrack(n, k, i + 1, current, result);
        // Unchoose
        current.pop();
    }
}

fn main() {
    let result = combinations_backtracking(4, 2);
    println!("{:?}", result);
    // Output: [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
}
