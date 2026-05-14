/**
 * Evaluate a mathematical expression with +, -, *, /, and parentheses.
 *
 * Approach: Recursive descent parser
 * - Parse expression level (+ and -)
 * - Parse term level (* and /)
 * - Parse factor level (numbers and parentheses)
 * - Respects operator precedence naturally
 *
 * Time: O(n) - single pass through string
 * Space: O(d) - recursion depth equals nesting level
 */

struct Calculator {
    s: Vec<char>,
    pos: usize,
}

impl Calculator {
    fn new(s: &str) -> Self {
        Calculator {
            s: s.chars().collect(),
            pos: 0,
        }
    }

    fn parse_number(&mut self) -> i32 {
        while self.pos < self.s.len() && self.s[self.pos].is_whitespace() {
            self.pos += 1;
        }
        let mut num = 0;
        while self.pos < self.s.len() && self.s[self.pos].is_ascii_digit() {
            num = num * 10 + (self.s[self.pos] as i32 - '0' as i32);
            self.pos += 1;
        }
        num
    }

    fn parse_factor(&mut self) -> i32 {
        while self.pos < self.s.len() && self.s[self.pos].is_whitespace() {
            self.pos += 1;
        }
        if self.s[self.pos] == '(' {
            self.pos += 1;  // Skip '('
            let result = self.parse_expression();
            while self.pos < self.s.len() && self.s[self.pos].is_whitespace() {
                self.pos += 1;
            }
            self.pos += 1;  // Skip ')'
            result
        } else {
            self.parse_number()
        }
    }

    fn parse_term(&mut self) -> i32 {
        let mut result = self.parse_factor();
        loop {
            while self.pos < self.s.len() && self.s[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos >= self.s.len() || (self.s[self.pos] != '*' && self.s[self.pos] != '/') {
                break;
            }
            let op = self.s[self.pos];
            self.pos += 1;
            let operand = self.parse_factor();
            if op == '*' {
                result *= operand;
            } else {
                result /= operand;
            }
        }
        result
    }

    fn parse_expression(&mut self) -> i32 {
        let mut result = self.parse_term();
        loop {
            while self.pos < self.s.len() && self.s[self.pos].is_whitespace() {
                self.pos += 1;
            }
            if self.pos >= self.s.len() || (self.s[self.pos] != '+' && self.s[self.pos] != '-') {
                break;
            }
            let op = self.s[self.pos];
            self.pos += 1;
            let operand = self.parse_term();
            if op == '+' {
                result += operand;
            } else {
                result -= operand;
            }
        }
        result
    }

    fn calculate(&mut self) -> i32 {
        self.parse_expression()
    }
}

fn basic_calculator_stack(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let mut calc = Calculator::new(s);
    calc.calculate()
}

fn main() {
    println!("{}", basic_calculator_stack("1 + 1"));  // 2
    println!("{}", basic_calculator_stack(" 2-1 + 2 "));  // 3
    println!("{}", basic_calculator_stack("(1+(4+5+2)-3)+(6+8)"));  // 23
    println!("{}", basic_calculator_stack("2*(5+5*2)/3+(6/2*5)"));  // 25
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_calculator() {
        assert_eq!(basic_calculator_stack("1 + 1"), 2);
        assert_eq!(basic_calculator_stack(" 2-1 + 2 "), 3);
        assert_eq!(basic_calculator_stack("(1+(4+5+2)-3)+(6+8)"), 23);
    }
}
