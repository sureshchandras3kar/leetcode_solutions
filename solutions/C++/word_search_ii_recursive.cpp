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

        std::vector<std::string> result;
        for (int i = 0; i < board.size(); i++) {
            for (int j = 0; j < board[0].size(); j++) {
                dfs(board, i, j, root, result);
            }
        }
        return result;
    }

private:
    void dfs(std::vector<std::vector<char>>& board, int i, int j, TrieNode* node, std::vector<std::string>& result) {
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
            int ni = i + dir[0], nj = j + dir[1];
            if (ni >= 0 && ni < board.size() && nj >= 0 && nj < board[0].size()) {
                dfs(board, ni, nj, next_node, result);
            }
        }
        board[i][j] = ch;
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
