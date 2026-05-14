#include <vector>
#include <string>
using namespace std;

bool exist(vector<vector<char>>& board, string word) {
    function<bool(int, int, int)> backtrack = [&](int row, int col, int index) {
        if (index == word.length()) return true;
        if (row < 0 || row >= board.size() || col < 0 || col >= board[0].size()) return false;
        if (board[row][col] != word[index]) return false;

        board[row][col] = '#';
        bool found = (backtrack(row + 1, col, index + 1) ||
                      backtrack(row - 1, col, index + 1) ||
                      backtrack(row, col + 1, index + 1) ||
                      backtrack(row, col - 1, index + 1));
        board[row][col] = word[index];
        return found;
    };

    for (int i = 0; i < board.size(); i++) {
        for (int j = 0; j < board[0].size(); j++) {
            if (backtrack(i, j, 0)) return true;
        }
    }
    return false;
}
