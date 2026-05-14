#include <algorithm>
#include <vector>
using namespace std;

class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        if (nums.empty()) return 0;
        sort(nums.begin(), nums.end());
        int best = 1, streak = 1;
        for (int i = 1; i < (int)nums.size(); i++) {
            if (nums[i] == nums[i - 1]) continue;  // skip duplicate
            if (nums[i] == nums[i - 1] + 1) {
                streak++;
                best = max(best, streak);
            } else {
                streak = 1;
            }
        }
        return best;
    }
};
