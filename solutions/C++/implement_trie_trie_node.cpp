/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node Class with unordered_map
Time: O(m) for insert, search, startsWith where m is key length
Space: O(ALPHABET_SIZE * N) where N is number of keys
*/

#include <unordered_map>
#include <string>
#include <iostream>

class TrieNode {
public:
    // Store children as hash map {char: TrieNode*}
    unordered_map<char, TrieNode*> children;
    // Mark if this node represents end of a word
    bool is_end_of_word = false;
};

class Trie {
private:
    TrieNode* root;

public:
    Trie() {
        root = new TrieNode();
    }

    // Insert a word into the trie. Time: O(m)
    void insert(string word) {
        TrieNode* node = root;
        for (char c : word) {
            if (node->children.find(c) == node->children.end()) {
                node->children[c] = new TrieNode();
            }
            node = node->children[c];
        }
        node->is_end_of_word = true;
    }

    // Search for exact word. Time: O(m)
    bool search(string word) {
        TrieNode* node = root;
        for (char c : word) {
            if (node->children.find(c) == node->children.end()) {
                return false;
            }
            node = node->children[c];
        }
        return node->is_end_of_word;
    }

    // Check if any word starts with given prefix. Time: O(m)
    bool startsWith(string prefix) {
        TrieNode* node = root;
        for (char c : prefix) {
            if (node->children.find(c) == node->children.end()) {
                return false;
            }
            node = node->children[c];
        }
        return true;
    }
};

// Usage
int main() {
    Trie trie;
    trie.insert("apple");
    cout << trie.search("apple") << endl;      // 1 (true)
    cout << trie.search("app") << endl;        // 0 (false)
    cout << trie.startsWith("app") << endl;    // 1 (true)
    trie.insert("app");
    cout << trie.search("app") << endl;        // 1 (true)
    return 0;
}
