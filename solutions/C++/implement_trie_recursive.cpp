#include <iostream>
#include <unordered_map>

struct TrieNode {
    std::unordered_map<char, TrieNode*> children;
    bool is_end_of_word = false;
};

class Trie {
private:
    TrieNode* root;
    TrieNode* findNode(const std::string& prefix) {
        TrieNode* node = root;
        for (char c : prefix) {
            if (node->children.find(c) == node->children.end()) {
                return nullptr;
            }
            node = node->children[c];
        }
        return node;
    }

public:
    Trie() {
        root = new TrieNode();
    }

    void insert(const std::string& word) {
        TrieNode* node = root;
        for (char c : word) {
            if (node->children.find(c) == node->children.end()) {
                node->children[c] = new TrieNode();
            }
            node = node->children[c];
        }
        node->is_end_of_word = true;
    }

    bool search(const std::string& word) {
        TrieNode* node = findNode(word);
        return node != nullptr && node->is_end_of_word;
    }

    bool startsWith(const std::string& prefix) {
        return findNode(prefix) != nullptr;
    }
};

int main() {
    Trie trie;
    trie.insert("apple");
    std::cout << trie.search("apple") << std::endl;   // 1
    std::cout << trie.search("app") << std::endl;     // 0
    std::cout << trie.startsWith("app") << std::endl; // 1
    return 0;
}
