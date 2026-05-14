#include <iostream>
#include <vector>
#include <algorithm>

std::vector<std::vector<int>> threeSumSortTwoPointer(std::vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    std::vector<std::vector<int>> result;
    int n = (int)nums.size();

    for (int i = 0; i < n - 2; i++) {
        if (nums[i] > 0) break;
        if (i > 0 && nums[i] == nums[i - 1]) continue;
        int left = i + 1, right = n - 1;
        while (left < right) {
            int s = nums[i] + nums[left] + nums[right];
            if (s == 0) {
                result.push_back({nums[i], nums[left], nums[right]});
                while (left < right && nums[left] == nums[left + 1]) left++;
                while (left < right && nums[right] == nums[right - 1]) right--;
                left++;
                right--;
            } else if (s < 0) {
                left++;
            } else {
                right--;
            }
        }
    }

    return result;
}

int main() {
    std::vector<int> nums = {-1, 0, 1, 2, -1, -4};
    std::vector<std::vector<int>> result = threeSumSortTwoPointer(nums);
    for (const auto& triplet : result) {
        std::cout << "[";
        for (int i = 0; i < (int)triplet.size(); i++) {
            std::cout << triplet[i];
            if (i + 1 < (int)triplet.size()) std::cout << ", ";
        }
        std::cout << "]\n";
    }
    return 0;
}
