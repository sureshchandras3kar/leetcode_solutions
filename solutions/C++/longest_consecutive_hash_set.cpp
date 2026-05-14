#include <unordered_set>
#include <vector>
using namespace std;

class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        unordered_set<int> num_set(nums.begin(), nums.end());
        int best = 0;
        for (int n : num_set) {
            if (!num_set.count(n - 1)) {  // start of a sequence
                int length = 1;
                while (num_set.count(n + length)) {
                    length++;
                }
                best = max(best, length);
            }
        }
        return best;
    }
};
