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
function validParenthesesReplace(s) {
    let prev;
    do {
        prev = s;
        s = s.replace(/\(\)/g, "").replace(/\[\]/g, "").replace(/\{\}/g, "");
    } while (s !== prev);

    return s.length === 0;
}

// Test cases
console.log(validParenthesesReplace("()"));  // true
console.log(validParenthesesReplace("()[]{}"));  // true
console.log(validParenthesesReplace("(]"));  // false
console.log(validParenthesesReplace("([])"));  // true
console.log(validParenthesesReplace("{[]}"));  // true
console.log(validParenthesesReplace(""));  // true
console.log(validParenthesesReplace("("));  // false
console.log(validParenthesesReplace(")"));  // false
