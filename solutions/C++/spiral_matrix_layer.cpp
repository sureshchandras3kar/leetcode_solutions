#include <iostream>
#include <vector>
#include <algorithm>

std::vector<int> spiralMatrixLayer(std::vector<std::vector<int>>& matrix) {
    if (matrix.empty() || matrix[0].empty()) {
        return {};
    }

    std::vector<int> result;
    int m = matrix.size();
    int n = matrix[0].size();
    int layers = (std::min(m, n) + 1) / 2;

    for (int layer = 0; layer < layers; layer++) {
        int top = layer;
        int bottom = m - 1 - layer;
        int left = layer;
        int right = n - 1 - layer;

        // Traverse right
        for (int col = left; col <= right; col++) {
            result.push_back(matrix[top][col]);
        }

        // Traverse down
        for (int row = top + 1; row <= bottom; row++) {
            result.push_back(matrix[row][right]);
        }

        // Traverse left (if there's a row remaining)
        if (top < bottom) {
            for (int col = right - 1; col >= left; col--) {
                result.push_back(matrix[bottom][col]);
            }
        }

        // Traverse up (if there's a column remaining)
        if (left < right) {
            for (int row = bottom - 1; row > top; row--) {
                result.push_back(matrix[row][left]);
            }
        }
    }

    return result;
}

int main() {
    std::vector<std::vector<int>> matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    std::vector<int> result = spiralMatrixLayer(matrix);
    for (int num : result) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    return 0;
}
