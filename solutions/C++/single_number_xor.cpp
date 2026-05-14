#include <vector>
using namespace std;

int singleNumberXor(vector<int>& nums) {
    int result = 0;
    for (int num : nums) {
        result ^= num;
    }
    return result;
}

int main() {
    vector<int> nums1 = {2, 2, 1};
    cout << singleNumberXor(nums1) << endl;  // 1
    return 0;
}
