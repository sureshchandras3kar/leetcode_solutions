#include <iostream>
#include <vector>
#include <algorithm>

int minSubArrayLenBinarySearch(int target, std::vector<int>& nums) {
    std::vector<int> prefix;
    prefix.push_back(0);
    for (int num : nums) {
        prefix.push_back(prefix.back() + num);
    }

    int min_length = INT_MAX;

    for (int right = 1; right < (int)prefix.size(); right++) {
        int needed = prefix[right] - target;
        int left = std::upper_bound(prefix.begin(), prefix.begin() + right, needed) - prefix.begin() - 1;

        if (left >= 0 && left < right) {
            min_length = std::min(min_length, right - left);
        }
    }

    return min_length == INT_MAX ? 0 : min_length;
}

int main() {
    std::vector<int> nums = {2, 3, 1, 2, 4, 3};
    int target = 7;
    int result = minSubArrayLenBinarySearch(target, nums);
    std::cout << result << std::endl;
    return 0;
}
