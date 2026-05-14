#include <vector>
using namespace std;

class Solution {
public:
    /**
     * Generate all combinations of k numbers from 1 to n using iterative approach.
     * Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
     */
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> result;
        vector<int> combo(k);

        // Initialize with [1, 2, ..., k]
        for (int i = 0; i < k; i++) {
            combo[i] = i + 1;
        }

        while (true) {
            result.push_back(combo);

            // Find the rightmost number that can be incremented
            int i = k - 1;
            while (i >= 0 && combo[i] == n - k + i + 1) {
                i--;
            }

            if (i < 0) break;

            // Increment and reset
            combo[i]++;
            for (int j = i + 1; j < k; j++) {
                combo[j] = combo[j - 1] + 1;
            }
        }

        return result;
    }
};

// Test
int main() {
    Solution sol;
    vector<vector<int>> result = sol.combine(4, 2);
    // Output: [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]
    return 0;
}
