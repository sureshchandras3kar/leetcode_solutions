/**
 * Check if parentheses are balanced by repeatedly removing matched pairs.
 *
 * Approach: Repeatedly remove consecutive pairs of matching brackets
 * until either the string is empty (valid) or no more pairs can be removed (invalid).
 *
 * Time: O(n²) - each pass removes at least 2 characters, up to n/2 passes
 * Space: O(n) - for temporary strings during replacement
 *
 * Note: This approach is simple to understand but slower than stack.
 */
fn valid_parentheses_replace(mut s: String) -> bool {
    let mut prev;
    loop {
        prev = s.clone();
        s = s.replace("()", "").replace("[]", "").replace("{}", "");
        if s == prev {
            break;
        }
    }

    s.is_empty()
}

fn main() {
    // Test cases
    println!("{}", valid_parentheses_replace("()".to_string()));  // true
    println!("{}", valid_parentheses_replace("()[]{}".to_string()));  // true
    println!("{}", valid_parentheses_replace("(]".to_string()));  // false
    println!("{}", valid_parentheses_replace("([])".to_string()));  // true
    println!("{}", valid_parentheses_replace("{[]}".to_string()));  // true
    println!("{}", valid_parentheses_replace("".to_string()));  // true
    println!("{}", valid_parentheses_replace("(".to_string()));  // false
    println!("{}", valid_parentheses_replace(")".to_string()));  // false
}
