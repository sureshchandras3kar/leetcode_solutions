#include <vector>
using namespace std;

class Solution {
public:
    /**
     * Generate all permutations using backtracking with used array.
     * Time: O(n! * n), Space: O(n!) for result
     */
    vector<vector<int>> permute(vector<int> nums) {
        vector<vector<int>> result;
        vector<bool> used(nums.size(), false);
        vector<int> current;
        backtrack(nums, used, current, result);
        return result;
    }

private:
    void backtrack(const vector<int>& nums, vector<bool>& used, vector<int>& current,
                   vector<vector<int>>& result) {
        // Base case: we've used all numbers
        if (current.size() == nums.size()) {
            result.push_back(current);
            return;
        }

        for (int i = 0; i < nums.size(); i++) {
            if (used[i]) continue;

            // Choose
            current.push_back(nums[i]);
            used[i] = true;
            // Explore
            backtrack(nums, used, current, result);
            // Unchoose
            current.pop_back();
            used[i] = false;
        }
    }
};

// Test
int main() {
    Solution sol;
    vector<vector<int>> result = sol.permute({1, 2, 3});
    // Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
    return 0;
}
