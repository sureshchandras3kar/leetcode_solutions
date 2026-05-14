#include <iostream>
#include <vector>

int removeDuplicates(std::vector<int>& nums) {
    /*
    Two-Pointer In-Place Approach
    Remove duplicates from sorted array in-place and return the length of the new array.

    Time Complexity: O(n)
    Space Complexity: O(1)
    */
    if (nums.empty()) {
        return 0;
    }

    int k = 1;  // First element is always unique
    for (int i = 1; i < (int)nums.size(); i++) {
        if (nums[i] != nums[i - 1]) {
            nums[k] = nums[i];
            k++;
        }
    }

    return k;
}

int main() {
    std::vector<int> nums1 = {1, 1, 2};
    std::cout << removeDuplicates(nums1) << std::endl;  // 2, nums = [1, 2, _]

    std::vector<int> nums2 = {0, 0, 1, 1, 1, 2, 2, 3, 3, 4};
    std::cout << removeDuplicates(nums2) << std::endl;  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]

    return 0;
}
