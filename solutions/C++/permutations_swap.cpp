#include <vector>
using namespace std;

class Solution {
public:
    /**
     * Generate all permutations using backtracking with swapping approach.
     * Time: O(n! * n), Space: O(n!) for result
     */
    vector<vector<int>> permute(vector<int> nums) {
        vector<vector<int>> result;
        backtrack(nums, 0, result);
        return result;
    }

private:
    void backtrack(vector<int>& nums, int first, vector<vector<int>>& result) {
        // Base case: all elements are placed
        if (first == nums.size()) {
            result.push_back(nums);
            return;
        }

        for (int i = first; i < nums.size(); i++) {
            // Swap
            swap(nums[first], nums[i]);
            // Backtrack
            backtrack(nums, first + 1, result);
            // Swap back
            swap(nums[first], nums[i]);
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
