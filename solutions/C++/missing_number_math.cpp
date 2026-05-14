#include <numeric>
#include <vector>
using namespace std;

class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int n = nums.size();
        int expected = n * (n + 1) / 2;
        return expected - accumulate(nums.begin(), nums.end(), 0);
    }
};
