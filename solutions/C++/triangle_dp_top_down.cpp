#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_map>

using namespace std;

unordered_map<string, int> memo;
vector<vector<int>> triangle_global;

string encode(int row, int col) {
    return to_string(row) + "," + to_string(col);
}

int dfs(int row, int col) {
    if (row == triangle_global.size() - 1) {
        return triangle_global[row][col];
    }

    string key = encode(row, col);
    if (memo.find(key) != memo.end()) {
        return memo[key];
    }

    int result = triangle_global[row][col] + min(
        dfs(row + 1, col),
        dfs(row + 1, col + 1)
    );
    memo[key] = result;
    return result;
}

int minimumTotal_top_down(vector<vector<int>>& triangle) {
    memo.clear();
    triangle_global = triangle;
    return dfs(0, 0);
}

int main() {
    vector<vector<int>> triangle = {{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}};
    cout << minimumTotal_top_down(triangle) << endl;  // 11
    return 0;
}
