#include <vector>
#include <iostream>
using namespace std;

/*
Simple Two Pointers Approach
Allow at most 2 occurrences by checking if current element differs from element 2 positions back.

Time Complexity: O(n)
Space Complexity: O(1)
*/
int removeDuplicates(vector<int>& nums) {
    if (nums.empty()) {
        return 0;
    }

    int k = 0;
    for (int i = 0; i < nums.size(); i++) {
        // Always write first 2 elements, or if current differs from element 2 positions back
        if (k < 2 || nums[i] != nums[k - 2]) {
            nums[k] = nums[i];
            k++;
        }
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
