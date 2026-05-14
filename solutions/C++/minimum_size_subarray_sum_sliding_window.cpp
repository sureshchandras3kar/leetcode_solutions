#include <iostream>
#include <vector>
#include <algorithm>

int minSubArrayLenSlidingWindow(int target, std::vector<int>& nums) {
    int left = 0;
    int current_sum = 0;
    int min_length = INT_MAX;

    for (int right = 0; right < (int)nums.size(); right++) {
        current_sum += nums[right];

        while (current_sum >= target) {
            min_length = std::min(min_length, right - left + 1);
            current_sum -= nums[left];
            left++;
        }
    }

    return min_length == INT_MAX ? 0 : min_length;
}

int main() {
    std::vector<int> nums = {2, 3, 1, 2, 4, 3};
    int target = 7;
    int result = minSubArrayLenSlidingWindow(target, nums);
    std::cout << result << std::endl;
    return 0;
}
