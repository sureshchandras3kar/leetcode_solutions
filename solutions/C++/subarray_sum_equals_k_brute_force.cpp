#include <vector>
using namespace std;

class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        int result = 0;
        for (int i = 0; i < (int)nums.size(); i++) {
            int total = 0;
            for (int j = i; j < (int)nums.size(); j++) {
                total += nums[j];
                if (total == k) result++;
            }
        }
        return result;
    }
};
