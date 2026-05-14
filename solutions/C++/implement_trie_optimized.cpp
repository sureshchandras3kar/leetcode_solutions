/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Optimized with Word Storage
Time: O(m) for insert, search, startsWith
Space: O(N * m) where N = number of words
Benefit: Can retrieve all words with common prefix
*/

#include <unordered_map>
#include <string>
#include <vector>
#include <iostream>

class TrieNode {
public:
    unordered_map<char, TrieNode*> children;
    bool is_end_of_word = false;
    string* word = nullptr;  // Store complete word at end nodes
};

class Trie {
private:
    TrieNode* root;

    TrieNode* findNode(const string& prefix) {
        TrieNode* node = root;
        for (char c : prefix) {
            if (node->children.find(c) == node->children.end()) {
                return nullptr;
            }
            node = node->children[c];
        }
        return node;
    }

    void dfsCollectWords(TrieNode* node, vector<string>& result) {
        if (node->is_end_of_word && node->word != nullptr) {
            result.push_back(*node->word);
        }
        for (auto& pair : node->children) {
            dfsCollectWords(pair.second, result);
        }
    }

public:
    Trie() {
        root = new TrieNode();
    }

    // Insert word with word storage. Time: O(m)
    void insert(string word) {
        TrieNode* node = root;
        for (char c : word) {
            if (node->children.find(c) == node->children.end()) {
                node->children[c] = new TrieNode();
            }
            node = node->children[c];
        }
        node->is_end_of_word = true;
        node->word = new string(word);
    }

    // Search for exact word. Time: O(m)
    bool search(string word) {
        TrieNode* node = findNode(word);
        return node != nullptr && node->is_end_of_word;
    }

    // Check if prefix exists. Time: O(m)
    bool startsWith(string prefix) {
        return findNode(prefix) != nullptr;
    }

    // Bonus: Get all words with given prefix. Time: O(n*m)
    vector<string> getAllWordsWithPrefix(const string& prefix) {
        vector<string> result;
        TrieNode* node = findNode(prefix);
        if (node == nullptr) {
            return result;
        }
        dfsCollectWords(node, result);
        return result;
    }
};

// Usage
int main() {
    Trie trie;
    trie.insert("apple");
    trie.insert("application");
    trie.insert("apply");

    cout << trie.search("apple") << endl;           // 1
    cout << trie.startsWith("app") << endl;         // 1

    vector<string> words = trie.getAllWordsWithPrefix("app");
    for (const string& w : words) {
        cout << w << " ";  // apple application apply
    }
    cout << endl;

    return 0;
}
