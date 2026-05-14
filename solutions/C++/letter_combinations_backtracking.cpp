#include <vector>
#include <string>
#include <unordered_map>
using namespace std;

class Solution {
public:
    /**
     * Generate all letter combinations using backtracking (recursive).
     * Time: O(4^n), Space: O(4^n) for result
     */
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return {};

        unordered_map<char, string> phoneMap = {
            {'2', "abc"},
            {'3', "def"},
            {'4', "ghi"},
            {'5', "jkl"},
            {'6', "mno"},
            {'7', "pqrs"},
            {'8', "tuv"},
            {'9', "wxyz"}
        };

        vector<string> result;
        backtrack(digits, 0, "", result, phoneMap);
        return result;
    }

private:
    void backtrack(const string& digits, int index, string current,
                   vector<string>& result,
                   const unordered_map<char, string>& phoneMap) {
        // Base case: we've processed all digits
        if (index == digits.length()) {
            result.push_back(current);
            return;
        }

        // Get the letters that the current digit maps to
        char currentDigit = digits[index];
        const string& letters = phoneMap.at(currentDigit);

        // Try each letter
        for (char letter : letters) {
            backtrack(digits, index + 1, current + letter, result, phoneMap);
        }
    }
};

// Test
int main() {
    Solution sol;
    vector<string> result = sol.letterCombinations("23");
    // Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    return 0;
}
