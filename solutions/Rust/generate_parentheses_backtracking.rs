pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = Vec::new();
    fn backtrack(result: &mut Vec<String>, current: String, left: i32, right: i32, n: i32) {
        if current.len() == (2 * n) as usize {
            result.push(current);
            return;
        }
        if left < n {
            backtrack(result, current.clone() + "(", left + 1, right, n);
        }
        if right < left {
            backtrack(result, current + ")", left, right + 1, n);
        }
    }
    backtrack(&mut result, String::new(), 0, 0, n);
    result
}
