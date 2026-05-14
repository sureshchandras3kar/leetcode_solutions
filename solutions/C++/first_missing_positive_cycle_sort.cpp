#include <vector>
using namespace std;

class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        int n = nums.size();
        int i = 0;

        // Place each number at its correct index (value v at index v-1)
        while (i < n) {
            int j = nums[i] - 1;
            if (nums[i] >= 1 && nums[i] <= n && nums[j] != nums[i]) {
                swap(nums[i], nums[j]);
            } else {
                i++;
            }
        }

        // Find first index where value doesn't match expected
        for (int i = 0; i < n; i++) {
            if (nums[i] != i + 1) {
                return i + 1;
            }
        }

        return n + 1;
    }
};
