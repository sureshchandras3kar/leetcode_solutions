#include <vector>
#include <unordered_set>
using namespace std;

int totalNQueens(int n) {
    function<int(int, unordered_set<int>&, unordered_set<int>&, unordered_set<int>&)> backtrack =
        [&](int row, unordered_set<int>& cols, unordered_set<int>& diag1, unordered_set<int>& diag2) {
        if (row == n) return 1;
        int count = 0;
        for (int col = 0; col < n; col++) {
            if (cols.count(col) == 0 && diag1.count(row - col) == 0 && diag2.count(row + col) == 0) {
                cols.insert(col);
                diag1.insert(row - col);
                diag2.insert(row + col);
                count += backtrack(row + 1, cols, diag1, diag2);
                cols.erase(col);
                diag1.erase(row - col);
                diag2.erase(row + col);
            }
        }
        return count;
    };

    unordered_set<int> cols, diag1, diag2;
    return backtrack(0, cols, diag1, diag2);
}
