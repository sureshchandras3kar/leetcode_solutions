#include <vector>
using namespace std;

/*
Remove all occurrences of a value in-place from an array and return the new length.

Approach: Brute Force
Scan the array. When an element equals val, remove it by erasing it from the vector.
This shifts all subsequent elements left by one position. Repeat until no more elements
equal val are found. Do not increment the index after removal.

Time Complexity: O(n²) — worst case when all elements equal val, each erase is O(n)
Space Complexity: O(1) — no extra data structures beyond the input vector

Example 1:
Input: nums = [3,2,2,3], val = 3
Output: 2
Array after: [2,2,_,_]
Explanation: First 3 is removed, then second 3 is removed.

Example 2:
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5
Array after: [0,1,4,0,3,_,_,_]
Explanation: Each 2 is removed by erasing and shifting subsequent elements left.
*/

class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int i = 0;
        while (i < nums.size()) {
            if (nums[i] == val) {
                nums.erase(nums.begin() + i);  // Remove element and shift all elements after i one position left
            } else {
                i++;
            }
        }
        return nums.size();
    }
};

/*
Test cases:
Solution sol;
vector<int> nums1 = {3, 2, 2, 3};
cout << sol.removeElement(nums1, 3) << endl;  // 2

vector<int> nums2 = {0, 1, 2, 2, 3, 0, 4, 2};
cout << sol.removeElement(nums2, 2) << endl;  // 5

vector<int> nums3 = {2};
cout << sol.removeElement(nums3, 3) << endl;  // 1

vector<int> nums4 = {};
cout << sol.removeElement(nums4, 0) << endl;  // 0
*/
