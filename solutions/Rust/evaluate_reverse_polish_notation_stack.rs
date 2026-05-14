/**
 * Evaluate Reverse Polish Notation using a stack.
 *
 * Time: O(n) - process each token once
 * Space: O(n) - stack stores operands
 */
pub fn evaluate_rpn_stack(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i64> = Vec::new();

    for token in tokens {
        match token.as_str() {
            "+" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                // Truncate towards zero
                stack.push(a / b);
            }
            _ => {
                // It's a number
                stack.push(token.parse::<i64>().unwrap());
            }
        }
    }

    stack[0] as i32
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let tokens = vec!["3".to_string(), "4".to_string(), "+".to_string()];
        assert_eq!(evaluate_rpn_stack(tokens), 7);
    }

    #[test]
    fn test_simple_multiplication() {
        let tokens = vec!["3".to_string(), "4".to_string(), "*".to_string()];
        assert_eq!(evaluate_rpn_stack(tokens), 12);
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
        assert_eq!(evaluate_rpn_stack(tokens), 9);
    }

    #[test]
    fn test_division() {
        let tokens = vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()];
        assert_eq!(evaluate_rpn_stack(tokens), 6);
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
        assert_eq!(evaluate_rpn_stack(tokens), 22);
    }
}

fn main() {
    let test1 = vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()];
    println!("{}", evaluate_rpn_stack(test1)); // 9

    let test2 = vec!["4".to_string(), "13".to_string(), "5".to_string(), "/".to_string(), "+".to_string()];
    println!("{}", evaluate_rpn_stack(test2)); // 6
}
