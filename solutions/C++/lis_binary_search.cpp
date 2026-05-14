#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Find length of longest increasing subsequence using binary search O(n log n).
 *
 * Time Complexity: O(n log n)
 * Space Complexity: O(n)
 */
int lisBinarySearch(vector<int>& nums) {
    if (nums.empty()) return 0;

    vector<int> tails;

    for (int num : nums) {
        auto it = lower_bound(tails.begin(), tails.end(), num);
        if (it == tails.end()) {
            tails.push_back(num);
        } else {
            *it = num;
        }
    }

    return tails.size();
}

int main() {
    vector<int> test1 = {10, 9, 2, 5, 3, 7, 101, 18};
    cout << lisBinarySearch(test1) << endl;  // 4
    return 0;
}
