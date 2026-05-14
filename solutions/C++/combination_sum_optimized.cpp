#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    /**
     * Find all unique combinations that sum to target using optimized backtracking.
     * Optimization: skip candidates that are too large.
     * Time: O(N^T), Space: O(T) for recursion depth
     */
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        sort(candidates.begin(), candidates.end());  // Sort to enable pruning
        vector<vector<int>> result;
        vector<int> current;
        backtrack(candidates, 0, current, target, result);
        return result;
    }

private:
    void backtrack(const vector<int>& candidates, int index, vector<int>& current,
                   int remaining, vector<vector<int>>& result) {
        // Base case: found a valid combination
        if (remaining == 0) {
            result.push_back(current);
            return;
        }

        // Explore: try each candidate from index onwards
        for (int i = index; i < candidates.size(); i++) {
            int candidate = candidates[i];

            // Pruning: if candidate > remaining, all further candidates will be too large
            if (candidate > remaining) {
                break;
            }

            // Choose
            current.push_back(candidate);
            // Explore: can reuse the same candidate (i, not i+1)
            backtrack(candidates, i, current, remaining - candidate, result);
            // Unchoose
            current.pop_back();
        }
    }
};

// Test
int main() {
    vector<int> candidates = {2, 3, 6, 7};
    Solution sol;
    vector<vector<int>> result = sol.combinationSum(candidates, 7);
    // Output: [[2,2,3], [7]]
    return 0;
}
