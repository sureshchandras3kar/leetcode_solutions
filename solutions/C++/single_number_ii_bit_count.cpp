#include <vector>
using namespace std;

int singleNumberIIBitCount(vector<int>& nums) {
    vector<int> bitCounts(32, 0);
    for (int num : nums) {
        for (int i = 0; i < 32; i++) {
            if (num & (1 << i)) {
                bitCounts[i]++;
            }
        }
    }

    int result = 0;
    for (int i = 0; i < 32; i++) {
        if (bitCounts[i] % 3) {
            result |= (1 << i);
        }
    }

    return result;
}

int main() {
    vector<int> nums = {2, 2, 3, 2};
    cout << singleNumberIIBitCount(nums) << endl;  // 3
    return 0;
}
