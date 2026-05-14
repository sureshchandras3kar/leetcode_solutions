#include <vector>
using namespace std;

class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        int rows[9] = {}, cols[9] = {}, boxes[9] = {};

        for (int r = 0; r < 9; ++r) {
            for (int c = 0; c < 9; ++c) {
                char d = board[r][c];
                if (d == '.') continue;
                int bit = 1 << (d - '1');
                int box = (r / 3) * 3 + (c / 3);
                if ((rows[r] & bit) || (cols[c] & bit) || (boxes[box] & bit))
                    return false;
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[box] |= bit;
            }
        }
        return true;
    }
};
