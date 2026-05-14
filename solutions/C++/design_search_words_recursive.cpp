#include <iostream>
#include <unordered_map>
#include <string>

struct TrieNode {
    std::unordered_map<char, TrieNode*> children;
    bool is_end_of_word = false;
};

class WordDictionary {
private:
    TrieNode* root;

    bool searchDFS(const std::string& word, int index, TrieNode* node) {
        if (index == (int)word.length()) {
            return node->is_end_of_word;
        }

        char ch = word[index];
        if (ch == '.') {
            for (auto& [c, child] : node->children) {
                if (searchDFS(word, index + 1, child)) {
                    return true;
                }
            }
            return false;
        } else {
            if (node->children.find(ch) == node->children.end()) {
                return false;
            }
            return searchDFS(word, index + 1, node->children[ch]);
        }
    }

public:
    WordDictionary() {
        root = new TrieNode();
    }

    void addWord(const std::string& word) {
        TrieNode* node = root;
        for (char ch : word) {
            if (node->children.find(ch) == node->children.end()) {
                node->children[ch] = new TrieNode();
            }
            node = node->children[ch];
        }
        node->is_end_of_word = true;
    }

    bool search(const std::string& word) {
        return searchDFS(word, 0, root);
    }
};

int main() {
    WordDictionary wd;
    wd.addWord("bad");
    wd.addWord("dad");
    wd.addWord("mad");
    std::cout << wd.search("pad") << std::endl;  // 0
    std::cout << wd.search("bad") << std::endl;  // 1
    std::cout << wd.search(".ad") << std::endl;  // 1
    std::cout << wd.search("b..") << std::endl;  // 1
    return 0;
}
