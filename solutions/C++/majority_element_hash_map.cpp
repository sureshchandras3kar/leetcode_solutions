#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
public:
    int majorityElement(vector<int>& nums) {
        unordered_map<int, int> counts;
        int threshold = nums.size() / 2;
        for (int num : nums) {
            counts[num]++;
            if (counts[num] > threshold) return num;
        }
        return nums[0];
    }
};
