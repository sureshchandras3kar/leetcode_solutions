#include <iostream>
#include <string>

bool validParenthesesReplace(std::string s) {
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
    bool changed = true;
    while (changed) {
        changed = false;
        size_t pos = 0;

        // Find and remove pairs
        while ((pos = s.find("()")) != std::string::npos) {
            s.erase(pos, 2);
            changed = true;
        }
        pos = 0;
        while ((pos = s.find("[]")) != std::string::npos) {
            s.erase(pos, 2);
            changed = true;
        }
        pos = 0;
        while ((pos = s.find("{}")) != std::string::npos) {
            s.erase(pos, 2);
            changed = true;
        }
    }

    return s.empty();
}

int main() {
    // Test cases
    std::cout << validParenthesesReplace("()") << std::endl;  // 1
    std::cout << validParenthesesReplace("()[]{}") << std::endl;  // 1
    std::cout << validParenthesesReplace("(]") << std::endl;  // 0
    std::cout << validParenthesesReplace("([])") << std::endl;  // 1
    std::cout << validParenthesesReplace("{[]}") << std::endl;  // 1
    std::cout << validParenthesesReplace("") << std::endl;  // 1
    std::cout << validParenthesesReplace("(") << std::endl;  // 0
    std::cout << validParenthesesReplace(")") << std::endl;  // 0
    return 0;
}
