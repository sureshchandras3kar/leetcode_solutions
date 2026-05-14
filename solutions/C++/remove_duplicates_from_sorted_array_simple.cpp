#include <iostream>
#include <vector>

int removeDuplicates(std::vector<int>& nums) {
    /*
    Simple Pass Approach
    Remove duplicates by iterating and comparing consecutive elements.

    Time Complexity: O(n)
    Space Complexity: O(1)
    */
    if (nums.empty()) {
        return 0;
    }

    int write_idx = 0;
    for (int read_idx = 1; read_idx < (int)nums.size(); read_idx++) {
        if (nums[read_idx] != nums[write_idx]) {
            write_idx++;
            nums[write_idx] = nums[read_idx];
        }
    }

    return write_idx + 1;
}

int main() {
    std::vector<int> nums1 = {1, 1, 2};
    std::cout << removeDuplicates(nums1) << std::endl;  // 2, nums = [1, 2, _]

    std::vector<int> nums2 = {0, 0, 1, 1, 1, 2, 2, 3, 3, 4};
    std::cout << removeDuplicates(nums2) << std::endl;  // 5, nums = [0, 1, 2, 3, 4, _, _, _, _, _]

    return 0;
}
