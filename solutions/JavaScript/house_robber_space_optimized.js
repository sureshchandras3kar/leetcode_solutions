/**
 * Rob houses with maximum money using space-optimized approach.
 *
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 */
function houseRobberSpaceOptimized(nums) {
    if (nums.length === 0) return 0;
    if (nums.length === 1) return nums[0];

    let prev2 = nums[0];
    let prev1 = Math.max(nums[0], nums[1]);

    for (let i = 2; i < nums.length; i++) {
        const current = Math.max(nums[i] + prev2, prev1);
        prev2 = prev1;
        prev1 = current;
    }

    return prev1;
}

console.log(houseRobberSpaceOptimized([1, 2, 3, 1]));      // 4
console.log(houseRobberSpaceOptimized([2, 7, 9, 3, 1]));   // 12
