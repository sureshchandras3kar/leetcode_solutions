#include <iostream>
#include <unordered_map>
#include <queue>
#include <string>

struct TrieNode {
    std::unordered_map<char, TrieNode*> children;
    bool is_end_of_word = false;
};

class WordDictionary {
private:
    TrieNode* root;

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
        std::queue<std::pair<TrieNode*, int>> q;
        q.push({root, 0});

        while (!q.empty()) {
            auto [node, index] = q.front();
            q.pop();

            if (index == (int)word.length()) {
                if (node->is_end_of_word) {
                    return true;
                }
                continue;
            }

            char ch = word[index];
            if (ch == '.') {
                for (auto& [c, child] : node->children) {
                    q.push({child, index + 1});
                }
            } else {
                if (node->children.find(ch) != node->children.end()) {
                    q.push({node->children[ch], index + 1});
                }
            }
        }
        return false;
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
