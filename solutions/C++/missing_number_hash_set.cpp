#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
public:
    int missingNumber(vector<int>& nums) {
        unordered_set<int> seen(nums.begin(), nums.end());
        for (int value = 0; value <= nums.size(); value++) {
            if (!seen.count(value)) return value;
        }
        return -1;
    }
};
