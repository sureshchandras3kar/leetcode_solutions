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
function validParenthesesStack(s) {
    const stack = [];
    const bracketMap = {
        '(': ')',
        '[': ']',
        '{': '}'
    };

    for (const char of s) {
        if (char in bracketMap) {
            // Opening bracket - push to stack
            stack.push(char);
        } else {
            // Closing bracket - check if it matches
            if (stack.length === 0 || bracketMap[stack.pop()] !== char) {
                return false;
            }
        }
    }

    // Stack should be empty (all brackets matched)
    return stack.length === 0;
}

// Test cases
console.log(validParenthesesStack("()"));  // true
console.log(validParenthesesStack("()[]{}"));  // true
console.log(validParenthesesStack("(]"));  // false
console.log(validParenthesesStack("([])"));  // true
console.log(validParenthesesStack("{[]}"));  // true
console.log(validParenthesesStack(""));  // true
console.log(validParenthesesStack("("));  // false
console.log(validParenthesesStack(")"));  // false
