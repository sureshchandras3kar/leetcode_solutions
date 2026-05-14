/**
 * Evaluate a mathematical expression with +, -, *, /, and parentheses.
 *
 * Approach: Two-Pass with precedence handling
 * - First pass: convert infix to postfix notation (Reverse Polish Notation)
 * - Second pass: evaluate postfix expression using a stack
 * - Handles operator precedence (* and / before + and -)
 *
 * Time: O(n) - two passes through string
 * Space: O(n) - stacks for postfix and evaluation
 */

fn precedence(op: char) -> u32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn basic_calculator_two_pass(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }

    // Tokenize the expression
    let mut tokens: Vec<String> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
        } else if chars[i].is_ascii_digit() {
            let mut num = 0;
            while i < chars.len() && chars[i].is_ascii_digit() {
                num = num * 10 + (chars[i] as i32 - '0' as i32);
                i += 1;
            }
            tokens.push(num.to_string());
        } else if "+-*/()".contains(chars[i]) {
            tokens.push(chars[i].to_string());
            i += 1;
        } else {
            i += 1;
        }
    }

    // Convert infix to postfix (Shunting Yard algorithm)
    let mut output: Vec<String> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();

    for token in tokens {
        if token.chars().next().unwrap().is_ascii_digit() {
            output.push(token);
        } else if token == "(" {
            op_stack.push(token);
        } else if token == ")" {
            while !op_stack.is_empty() && op_stack.last().unwrap() != "(" {
                output.push(op_stack.pop().unwrap());
            }
            if !op_stack.is_empty() {
                op_stack.pop();  // Remove '('
            }
        } else if "+-*/".contains(&token) {
            while !op_stack.is_empty() &&
                  op_stack.last().unwrap() != "(" &&
                  precedence(op_stack.last().unwrap().chars().next().unwrap()) >=
                  precedence(token.chars().next().unwrap()) {
                output.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    while !op_stack.is_empty() {
        output.push(op_stack.pop().unwrap());
    }

    // Evaluate postfix expression
    let mut eval_stack: Vec<i64> = Vec::new();
    for token in output {
        if token.chars().next().unwrap().is_ascii_digit() {
            eval_stack.push(token.parse().unwrap());
        } else {
            let b = eval_stack.pop().unwrap_or(0);
            let a = eval_stack.pop().unwrap_or(0);
            match token.as_str() {
                "+" => eval_stack.push(a + b),
                "-" => eval_stack.push(a - b),
                "*" => eval_stack.push(a * b),
                "/" => eval_stack.push(a / b),
                _ => {}
            }
        }
    }

    *eval_stack.last().unwrap_or(&0) as i32
}

fn main() {
    println!("{}", basic_calculator_two_pass("1 + 1"));  // 2
    println!("{}", basic_calculator_two_pass(" 2-1 + 2 "));  // 3
    println!("{}", basic_calculator_two_pass("(1+(4+5+2)-3)+(6+8)"));  // 23
    println!("{}", basic_calculator_two_pass("2*(5+5*2)/3+(6/2*5)"));  // 17
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_calculator() {
        assert_eq!(basic_calculator_two_pass("1 + 1"), 2);
        assert_eq!(basic_calculator_two_pass(" 2-1 + 2 "), 3);
        assert_eq!(basic_calculator_two_pass("(1+(4+5+2)-3)+(6+8)"), 23);
    }
}
