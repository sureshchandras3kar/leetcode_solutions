/**
 * Evaluate Reverse Polish Notation using recursion.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - recursion stack depth
 */
pub fn evaluate_rpn_recursive(tokens: Vec<String>) -> i32 {
    let mut index = tokens.len() as i32 - 1;

    fn helper(tokens: &[String], index: &mut i32) -> i64 {
        let token = tokens[*index as usize].clone();
        *index -= 1;

        match token.as_str() {
            "+" => {
                let b = helper(tokens, index);
                let a = helper(tokens, index);
                a + b
            }
            "-" => {
                let b = helper(tokens, index);
                let a = helper(tokens, index);
                a - b
            }
            "*" => {
                let b = helper(tokens, index);
                let a = helper(tokens, index);
                a * b
            }
            "/" => {
                let b = helper(tokens, index);
                let a = helper(tokens, index);
                a / b
            }
            _ => token.parse::<i64>().unwrap(),
        }
    }

    helper(&tokens, &mut index) as i32
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let tokens = vec!["3".to_string(), "4".to_string(), "+".to_string()];
        assert_eq!(evaluate_rpn_recursive(tokens), 7);
    }

    #[test]
    fn test_simple_multiplication() {
        let tokens = vec!["3".to_string(), "4".to_string(), "*".to_string()];
        assert_eq!(evaluate_rpn_recursive(tokens), 12);
    }

    #[test]
    fn test_complex_expression() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        assert_eq!(evaluate_rpn_recursive(tokens), 9);
    }

    #[test]
    fn test_division() {
        let tokens = vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()];
        assert_eq!(evaluate_rpn_recursive(tokens), 6);
    }

    #[test]
    fn test_complex_mixed() {
        let tokens = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];
        assert_eq!(evaluate_rpn_recursive(tokens), 22);
    }
}

fn main() {
    let test1 = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
    println!("{}", evaluate_rpn_recursive(test1)); // 9

    let test2 = vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()];
    println!("{}", evaluate_rpn_recursive(test2)); // 6
}
