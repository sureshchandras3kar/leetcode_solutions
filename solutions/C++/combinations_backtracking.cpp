#include <vector>
using namespace std;

class Solution {
public:
    /**
     * Generate all combinations of k numbers from 1 to n using backtracking.
     * Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
     */
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> result;
        vector<int> current;
        backtrack(n, k, 1, current, result);
        return result;
    }

private:
    void backtrack(int n, int k, int start, vector<int>& current,
                   vector<vector<int>>& result) {
        // Base case: we've selected k numbers
        if (current.size() == k) {
            result.push_back(current);
            return;
        }

        // Explore: try each remaining number
        for (int i = start; i <= n; i++) {
            // Choose
            current.push_back(i);
            // Explore
            backtrack(n, k, i + 1, current, result);
            // Unchoose
            current.pop_back();
        }
    }
};

// Test
int main() {
    Solution sol;
    vector<vector<int>> result = sol.combine(4, 2);
    // Output: [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]
    return 0;
}
