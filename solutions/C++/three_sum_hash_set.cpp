#include <iostream>
#include <vector>
#include <unordered_set>
#include <algorithm>

std::vector<std::vector<int>> threeSumHashSet(std::vector<int>& nums) {
    std::sort(nums.begin(), nums.end());
    std::vector<std::vector<int>> result;
    int n = (int)nums.size();

    for (int i = 0; i < n - 2; i++) {
        if (i > 0 && nums[i] == nums[i - 1]) continue;
        std::unordered_set<int> seen;
        for (int j = i + 1; j < n; j++) {
            int need = -(nums[i] + nums[j]);
            if (seen.count(need)) {
                std::vector<int> triplet = {nums[i], need, nums[j]};
                if (std::find(result.begin(), result.end(), triplet) == result.end()) {
                    result.push_back(triplet);
                }
            }
            seen.insert(nums[j]);
        }
    }

    return result;
}

int main() {
    std::vector<int> nums = {-1, 0, 1, 2, -1, -4};
    std::vector<std::vector<int>> result = threeSumHashSet(nums);
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
