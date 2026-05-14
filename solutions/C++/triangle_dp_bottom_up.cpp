#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int minimumTotal_bottom_up(vector<vector<int>>& triangle) {
    /*
    DP Bottom-Up approach for Triangle problem.
    Builds the solution from the bottom row upward.
    Time: O(n²), Space: O(n)
    */
    int n = triangle.size();
    vector<int> dp = triangle[n - 1];  // Copy last row

    // Work backwards from the second-to-last row to the top
    for (int i = n - 2; i >= 0; --i) {
        for (int j = 0; j < triangle[i].size(); ++j) {
            dp[j] = triangle[i][j] + min(dp[j], dp[j + 1]);
        }
    }

    return dp[0];
}

int main() {
    vector<vector<int>> triangle = {{2}, {3, 4}, {6, 5, 7}, {4, 1, 8, 3}};
    cout << minimumTotal_bottom_up(triangle) << endl;  // 11
    return 0;
}
