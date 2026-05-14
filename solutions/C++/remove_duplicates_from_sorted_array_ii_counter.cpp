#include <vector>
#include <iostream>
using namespace std;

/*
Two Pointers with Counter Approach
Allow each element to appear at most twice using explicit count tracking.

Time Complexity: O(n)
Space Complexity: O(1)
*/
int removeDuplicates(vector<int>& nums) {
    if (nums.empty()) {
        return 0;
    }

    int k = 1;
    int count = 1;
    for (int i = 1; i < nums.size(); i++) {
        if (nums[i] != nums[i - 1]) {
            // New element encountered, reset counter
            count = 1;
            nums[k] = nums[i];
            k++;
        } else if (count < 2) {
            // Same element but less than 2 occurrences, allow it
            count++;
            nums[k] = nums[i];
            k++;
        }
        // else: count == 2, skip this duplicate
    }

    return k;
}

int main() {
    vector<int> nums1 = {1, 1, 1, 2, 2, 3};
    cout << removeDuplicates(nums1) << endl;  // 5, nums = [1, 1, 2, 2, 3, _]

    vector<int> nums2 = {0, 0, 1, 1, 1, 1, 2, 3, 3};
    cout << removeDuplicates(nums2) << endl;  // 7, nums = [0, 0, 1, 1, 2, 3, 3, _, _]

    vector<int> nums3 = {1};
    cout << removeDuplicates(nums3) << endl;  // 1, nums = [1]

    vector<int> nums4 = {1, 2};
    cout << removeDuplicates(nums4) << endl;  // 2, nums = [1, 2]

    return 0;
}
