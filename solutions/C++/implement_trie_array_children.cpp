/*
LeetCode #208: Implement Trie (Prefix Tree)
Approach: Trie Node with Fixed-Size Array (26 letters)
Time: O(m) for insert, search, startsWith
Space: O(ALPHABET_SIZE * N) where ALPHABET_SIZE = 26
*/

#include <array>
#include <string>
#include <iostream>

const int ALPHABET_SIZE = 26;

class TrieNode {
public:
    std::array<TrieNode*, ALPHABET_SIZE> children;
    bool is_end_of_word = false;

    TrieNode() {
        for (int i = 0; i < ALPHABET_SIZE; i++) {
            children[i] = nullptr;
        }
    }

    int charToIndex(char c) const {
        return c - 'a';
    }
};

class Trie {
private:
    TrieNode* root;

public:
    Trie() {
        root = new TrieNode();
    }

    // Insert word into trie. Time: O(m)
    void insert(string word) {
        TrieNode* node = root;
        for (char c : word) {
            int idx = node->charToIndex(c);
            if (node->children[idx] == nullptr) {
                node->children[idx] = new TrieNode();
            }
            node = node->children[idx];
        }
        node->is_end_of_word = true;
    }

    // Search for exact word. Time: O(m)
    bool search(string word) {
        TrieNode* node = root;
        for (char c : word) {
            int idx = node->charToIndex(c);
            if (node->children[idx] == nullptr) {
                return false;
            }
            node = node->children[idx];
        }
        return node->is_end_of_word;
    }

    // Check if any word starts with prefix. Time: O(m)
    bool startsWith(string prefix) {
        TrieNode* node = root;
        for (char c : prefix) {
            int idx = node->charToIndex(c);
            if (node->children[idx] == nullptr) {
                return false;
            }
            node = node->children[idx];
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
    return 0;
}
