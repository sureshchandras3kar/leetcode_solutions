pub fn combination_sum_basic(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut current = vec![];
    backtrack(&candidates, 0, &mut current, target, &mut result);
    result
}

fn backtrack(candidates: &Vec<i32>, index: usize, current: &mut Vec<i32>,
             remaining: i32, result: &mut Vec<Vec<i32>>) {
    // Base case: found a valid combination
    if remaining == 0 {
        result.push(current.clone());
        return;
    }

    // Base case: no valid combinations possible
    if remaining < 0 {
        return;
    }

    // Explore: try each candidate from index onwards
    for i in index..candidates.len() {
        let candidate = candidates[i];
        // Choose
        current.push(candidate);
        // Explore: can reuse the same candidate (i, not i+1)
        backtrack(candidates, i, current, remaining - candidate, result);
        // Unchoose
        current.pop();
    }
}

fn main() {
    let result = combination_sum_basic(vec![2, 3, 6, 7], 7);
    println!("{:?}", result);
    // Output: [[2, 2, 3], [7]]
}
