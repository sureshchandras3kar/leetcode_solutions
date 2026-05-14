#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        unordered_map<int, int> count;
        count[0] = 1;
        int prefix = 0, result = 0;
        for (int num : nums) {
            prefix += num;
            auto it = count.find(prefix - k);
            if (it != count.end()) {
                result += it->second;
            }
            count[prefix]++;
        }
        return result;
    }
};
