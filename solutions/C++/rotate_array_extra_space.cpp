#include <iostream>
#include <vector>

void rotateArrayExtraSpace(std::vector<int>& nums, int k) {
    /**
     * Rotate array using extra space.
     *
     * Time: O(n) | Space: O(n)
     *
     * Approach: Create a new array where element at index i goes to index
     * (i + k) % n. Copy back to original array.
     */
    if (nums.empty() || k == 0) {
        return;
    }

    int n = nums.size();
    k = k % n;  // Handle k > n
    if (k == 0) {
        return;
    }

    // Create rotated result
    std::vector<int> rotated(n);
    for (int i = 0; i < n; i++) {
        rotated[(i + k) % n] = nums[i];
    }

    // Copy back to original array
    for (int i = 0; i < n; i++) {
        nums[i] = rotated[i];
    }
}

int main() {
    std::vector<int> nums = {1, 2, 3, 4, 5};
    rotateArrayExtraSpace(nums, 2);
    for (int num : nums) {
        std::cout << num << " ";
    }
    std::cout << std::endl;  // [4, 5, 1, 2, 3]
    return 0;
}
