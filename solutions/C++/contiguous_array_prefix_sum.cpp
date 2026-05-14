#include <unordered_map>
#include <vector>
using namespace std;

class Solution {
public:
    int findMaxLength(vector<int>& nums) {
        unordered_map<int, int> seen;
        seen[0] = -1;
        int prefix = 0, best = 0;
        for (int i = 0; i < (int)nums.size(); i++) {
            prefix += nums[i] == 1 ? 1 : -1;
            auto it = seen.find(prefix);
            if (it != seen.end()) {
                best = max(best, i - it->second);
            } else {
                seen[prefix] = i;
            }
        }
        return best;
    }
};
