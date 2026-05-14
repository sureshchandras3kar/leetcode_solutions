use std::collections::HashMap;

/**
 * Check if parentheses are balanced using a stack.
 *
 * Approach: Use a stack to match closing brackets with opening brackets.
 * - Push opening brackets onto the stack
 * - For each closing bracket, check if it matches the top of the stack
 * - At the end, the stack should be empty
 *
 * Time: O(n) - single pass through the string
 * Space: O(n) - stack size in worst case (all opening brackets)
 */
fn valid_parentheses_stack(s: &str) -> bool {
    let mut stack = Vec::new();
    let mut bracket_map = HashMap::new();
    bracket_map.insert('(', ')');
    bracket_map.insert('[', ']');
    bracket_map.insert('{', '}');

    for c in s.chars() {
        if bracket_map.contains_key(&c) {
            // Opening bracket - push to stack
            stack.push(c);
        } else {
            // Closing bracket - check if it matches
            if let Some(last) = stack.pop() {
                if bracket_map.get(&last) != Some(&c) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    // Stack should be empty (all brackets matched)
    stack.is_empty()
}

fn main() {
    // Test cases
    println!("{}", valid_parentheses_stack("()"));  // true
    println!("{}", valid_parentheses_stack("()[]{}"));  // true
    println!("{}", valid_parentheses_stack("(]"));  // false
    println!("{}", valid_parentheses_stack("([])"));  // true
    println!("{}", valid_parentheses_stack("{[]}"));  // true
    println!("{}", valid_parentheses_stack(""));  // true
    println!("{}", valid_parentheses_stack("("));  // false
    println!("{}", valid_parentheses_stack(")"));  // false
}
