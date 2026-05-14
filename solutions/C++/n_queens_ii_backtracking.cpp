#include <vector>
using namespace std;

int totalNQueens(int n) {
    vector<int> board(n, -1);

    function<bool(int, int)> is_safe = [&](int row, int col) {
        for (int i = 0; i < row; i++) {
            if (board[i] == col || abs(board[i] - col) == abs(i - row))
                return false;
        }
        return true;
    };

    function<int(int)> backtrack = [&](int row) -> int {
        if (row == n) return 1;
        int count = 0;
        for (int col = 0; col < n; col++) {
            if (is_safe(row, col)) {
                board[row] = col;
                count += backtrack(row + 1);
            }
        }
        return count;
    };

    return backtrack(0);
}
