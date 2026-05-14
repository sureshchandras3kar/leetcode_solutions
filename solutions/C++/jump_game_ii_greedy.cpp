#include <iostream>
#include <vector>
#include <algorithm>

int jumpGameIIGreedy(std::vector<int>& nums) {
    /*
    Greedy approach: track the farthest reachable index and jump count.

    Intuition: We maintain the range [current_start, current_end] that can be reached
    with the current number of jumps. When we exhaust this range, we increment jumps
    and expand to [current_end + 1, farthest], which is reachable with one more jump.

    Time Complexity: O(n) - single pass through array
    Space Complexity: O(1) - only use constant extra space
    */
    if (nums.size() <= 1) {
        return 0;
    }

    int jumps = 0;
    int currentEnd = 0;   // End of range reachable with current number of jumps
    int farthest = 0;     // Farthest index reachable so far

    for (int i = 0; i < (int)nums.size() - 1; ++i) {
        // Update the farthest index we can reach
        farthest = std::max(farthest, i + nums[i]);

        // If we've reached the end of current jump range, we must jump
        if (i == currentEnd) {
            jumps++;
            currentEnd = farthest;

            // Optimization: if we can reach the end, no need to continue
            if (currentEnd >= (int)nums.size() - 1) {
                break;
            }
        }
    }

    return jumps;
}

int main() {
    std::vector<int> test1 = {2, 3, 1, 1, 4};
    std::cout << jumpGameIIGreedy(test1) << std::endl;  // 2

    std::vector<int> test2 = {2, 3, 0, 6, 9, 1, 2};
    std::cout << jumpGameIIGreedy(test2) << std::endl;  // 2

    std::vector<int> test3 = {10};
    std::cout << jumpGameIIGreedy(test3) << std::endl;  // 0

    std::vector<int> test4 = {1, 1, 1, 0};
    std::cout << jumpGameIIGreedy(test4) << std::endl;  // 3

    return 0;
}
