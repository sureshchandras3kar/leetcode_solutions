/**
 * Greedy approach: work backwards from the end to find the furthest index
 * we can reach. If we can reach index 0, we can reach the end.
 *
 * Time: O(n), Space: O(1)
 */
function canJumpGreedy(nums) {
    let lastGoodIndex = nums.length - 1;
    for (let i = nums.length - 2; i >= 0; i--) {
        if (i + nums[i] >= lastGoodIndex) {
            lastGoodIndex = i;
        }
    }
    return lastGoodIndex === 0;
}

// Test cases
console.log(canJumpGreedy([2, 3, 1, 1, 4]));  // true
console.log(canJumpGreedy([3, 2, 1, 0, 4]));  // false
console.log(canJumpGreedy([0]));              // true
console.log(canJumpGreedy([2, 0, 0]));        // true
console.log(canJumpGreedy([0, 2, 3]));        // false
