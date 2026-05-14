#include <vector>
#include <string>
#include <queue>
#include <unordered_map>
using namespace std;

class Solution {
public:
    /**
     * Generate all letter combinations using BFS (iterative).
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

        queue<string> q;
        q.push("");

        for (char digit : digits) {
            int size = q.size();
            const string& letters = phoneMap[digit];

            // Process all current combinations
            for (int i = 0; i < size; i++) {
                string current = q.front();
                q.pop();

                // Create a new combination for each letter
                for (char letter : letters) {
                    q.push(current + letter);
                }
            }
        }

        vector<string> result;
        while (!q.empty()) {
            result.push_back(q.front());
            q.pop();
        }
        return result;
    }
};

// Test
int main() {
    Solution sol;
    vector<string> result = sol.letterCombinations("23");
    // Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    return 0;
}
