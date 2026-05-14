#include <vector>
#include <algorithm>
using namespace std;

bool searchMatrix(vector<vector<int>>& matrix, int target) {
    if (matrix.empty() || matrix[0].empty()) return false;

    int row_count = matrix.size();
    for (auto& row : matrix) {
        if (binary_search(row.begin(), row.end(), target)) return true;
    }
    return false;
}
