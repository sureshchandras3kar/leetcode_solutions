#include <iostream>
#include <vector>
#include <algorithm>

std::vector<int> twoSumTwoPointer(std::vector<int> nums, int target) {
    std::sort(nums.begin(), nums.end());
    int left = 0, right = (int)nums.size() - 1;
    while (left < right) {
        int currentSum = nums[left] + nums[right];
        if (currentSum == target) {
            return {nums[left], nums[right]};
        } else if (currentSum < target) {
            left++;
        } else {
            right--;
        }
    }
    return {};
}

int main() {
    std::vector<int> nums = {2, 7, 11, 15};
    int target = 9;
    std::vector<int> result = twoSumTwoPointer(nums, target);
    for (int num : result) {
        std::cout << num << " ";
    }
    std::cout << std::endl;
    return 0;
}
