#include <iostream>
#include <vector>

void rotateArrayReverse(std::vector<int>& nums, int k) {
    /**
     * Rotate array in-place using reverse trick.
     *
     * Time: O(n) | Space: O(1)
     *
     * Approach: Reverse the entire array, then reverse first k elements,
     * then reverse remaining n-k elements. This achieves rotation without
     * extra space.
     */
    if (nums.empty() || k == 0) {
        return;
    }

    int n = nums.size();
    k = k % n;  // Handle k > n
    if (k == 0) {
        return;
    }

    auto reverse = [&](int start, int end) {
        while (start < end) {
            std::swap(nums[start], nums[end]);
            start++;
            end--;
        }
    };

    // Reverse entire array: [1,2,3,4,5] -> [5,4,3,2,1]
    reverse(0, n - 1);
    // Reverse first k: [5,4,3,2,1] -> [3,4,5,2,1]
    reverse(0, k - 1);
    // Reverse rest: [3,4,5,2,1] -> [3,4,5,1,2]
    reverse(k, n - 1);
}

int main() {
    std::vector<int> nums = {1, 2, 3, 4, 5};
    rotateArrayReverse(nums, 2);
    for (int num : nums) {
        std::cout << num << " ";
    }
    std::cout << std::endl;  // [4, 5, 1, 2, 3]
    return 0;
}
