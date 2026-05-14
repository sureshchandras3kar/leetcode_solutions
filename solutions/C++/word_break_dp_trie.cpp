#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

struct TrieNode {
    unordered_map<char, TrieNode*> children;
    bool is_end = false;
};

/**
 * Determine if string can be segmented using words from dictionary (Trie approach).
 *
 * Time Complexity: O(n*m)
 * Space Complexity: O(n + T)
 */
bool wordBreakDpTrie(const string& s, const vector<string>& word_dict) {
    TrieNode* root = new TrieNode();
    for (const string& word : word_dict) {
        TrieNode* node = root;
        for (char ch : word) {
            if (!node->children.count(ch)) {
                node->children[ch] = new TrieNode();
            }
            node = node->children[ch];
        }
        node->is_end = true;
    }

    vector<bool> dp(s.length() + 1, false);
    dp[0] = true;

    for (int i = 1; i <= s.length(); i++) {
        if (!dp[i]) {
            TrieNode* node = root;
            for (int j = i - 1; j >= 0; j--) {
                if (!node->children.count(s[j])) {
                    break;
                }
                node = node->children[s[j]];
                if (dp[j] && node->is_end) {
                    dp[i] = true;
                    break;
                }
            }
        }
    }

    return dp[s.length()];
}

int main() {
    cout << wordBreakDpTrie("leetcode", {"leet", "code"}) << endl;  // 1
    cout << wordBreakDpTrie("applepenapple", {"apple", "pen"}) << endl;  // 1
    cout << wordBreakDpTrie("catsandog", {"cat", "cats", "and", "sand", "dog"}) << endl;  // 0
    return 0;
}
