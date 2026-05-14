#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        unordered_set<char> rows[9], cols[9], boxes[9];

        for (int r = 0; r < 9; ++r) {
            for (int c = 0; c < 9; ++c) {
                char d = board[r][c];
                if (d == '.') continue;
                int box = (r / 3) * 3 + (c / 3);
                if (rows[r].count(d) || cols[c].count(d) || boxes[box].count(d))
                    return false;
                rows[r].insert(d);
                cols[c].insert(d);
                boxes[box].insert(d);
            }
        }
        return true;
    }
};
