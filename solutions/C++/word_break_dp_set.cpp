#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>

using namespace std;

/**
 * Determine if string can be segmented using words from dictionary.
 *
 * Time Complexity: O(n²)
 * Space Complexity: O(n + m)
 */
bool wordBreakDpSet(const string& s, const vector<string>& word_dict) {
    unordered_set<string> word_set(word_dict.begin(), word_dict.end());
    vector<bool> dp(s.length() + 1, false);
    dp[0] = true;

    for (int i = 1; i <= s.length(); i++) {
        for (int j = 0; j < i; j++) {
            if (dp[j] && word_set.count(s.substr(j, i - j))) {
                dp[i] = true;
                break;
            }
        }
    }

    return dp[s.length()];
}

int main() {
    cout << wordBreakDpSet("leetcode", {"leet", "code"}) << endl;  // 1
    cout << wordBreakDpSet("applepenapple", {"apple", "pen"}) << endl;  // 1
    cout << wordBreakDpSet("catsandog", {"cat", "cats", "and", "sand", "dog"}) << endl;  // 0
    return 0;
}
