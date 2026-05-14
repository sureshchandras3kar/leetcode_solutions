#include <iostream>
#include <stack>
#include <string>
#include <unordered_map>

bool validParenthesesStack(std::string s) {
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
    std::stack<char> st;
    std::unordered_map<char, char> bracket_map = {
        {'(', ')'}, {'[', ']'}, {'{', '}'}
    };

    for (char c : s) {
        if (bracket_map.count(c)) {
            // Opening bracket - push to stack
            st.push(c);
        } else {
            // Closing bracket - check if it matches
            if (st.empty() || bracket_map[st.top()] != c) {
                return false;
            }
            st.pop();
        }
    }

    // Stack should be empty (all brackets matched)
    return st.empty();
}

int main() {
    // Test cases
    std::cout << validParenthesesStack("()") << std::endl;  // 1
    std::cout << validParenthesesStack("()[]{}") << std::endl;  // 1
    std::cout << validParenthesesStack("(]") << std::endl;  // 0
    std::cout << validParenthesesStack("([])") << std::endl;  // 1
    std::cout << validParenthesesStack("{[]}") << std::endl;  // 1
    std::cout << validParenthesesStack("") << std::endl;  // 1
    std::cout << validParenthesesStack("(") << std::endl;  // 0
    std::cout << validParenthesesStack(")") << std::endl;  // 0
    return 0;
}
