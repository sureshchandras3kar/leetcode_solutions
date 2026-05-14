#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <unordered_set>

struct TrieNode {
    std::unordered_map<char, TrieNode*> children;
    std::string word = "";
};

class Solution {
private:
    std::vector<std::string> result;
    std::unordered_set<std::pair<int, int>, std::hash<std::pair<int, int>>> visited;

    void backtrack(std::vector<std::vector<char>>& board, int i, int j, TrieNode* node) {
        if (i < 0 || i >= board.size() || j < 0 || j >= board[0].size()) {
            return;
        }

        char ch = board[i][j];
        if (node->children.find(ch) == node->children.end()) {
            return;
        }

        TrieNode* next_node = node->children[ch];
        if (!next_node->word.empty()) {
            result.push_back(next_node->word);
            next_node->word = "";
        }

        board[i][j] = '#';
        int dirs[4][2] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
        for (auto& dir : dirs) {
            backtrack(board, i + dir[0], j + dir[1], next_node);
        }
        board[i][j] = ch;
    }

public:
    std::vector<std::string> findWords(std::vector<std::vector<char>>& board, std::vector<std::string>& words) {
        TrieNode* root = new TrieNode();
        for (const auto& word : words) {
            TrieNode* node = root;
            for (char ch : word) {
                if (node->children.find(ch) == node->children.end()) {
                    node->children[ch] = new TrieNode();
                }
                node = node->children[ch];
            }
            node->word = word;
        }

        for (int i = 0; i < board.size(); i++) {
            for (int j = 0; j < board[0].size(); j++) {
                backtrack(board, i, j, root);
            }
        }
        return result;
    }
};

int main() {
    Solution sol;
    std::vector<std::vector<char>> board = {
        {'o', 'a', 'a', 'n'},
        {'e', 't', 'a', 'e'},
        {'i', 'h', 'k', 'r'},
        {'i', 'f', 'l', 'v'}
    };
    std::vector<std::string> words = {"oath", "pea", "eat", "rain"};
    auto result = sol.findWords(board, words);
    for (const auto& word : result) {
        std::cout << word << " ";
    }
    std::cout << std::endl;
    return 0;
}
