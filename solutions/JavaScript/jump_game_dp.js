/**
 * Dynamic programming approach: forward pass tracking the furthest
 * reachable index. If we can ever reach the end index, return true.
 *
 * Time: O(n), Space: O(1)
 */
function canJumpDP(nums) {
    let maxReach = 0;
    for (let i = 0; i < nums.length; i++) {
        if (i > maxReach) {
            return false;
        }
        maxReach = Math.max(maxReach, i + nums[i]);
        if (maxReach >= nums.length - 1) {
            return true;
        }
    }
    return false;
}

// Test cases
console.log(canJumpDP([2, 3, 1, 1, 4]));  // true
console.log(canJumpDP([3, 2, 1, 0, 4]));  // false
console.log(canJumpDP([0]));              // true
console.log(canJumpDP([2, 0, 0]));        // true
console.log(canJumpDP([0, 2, 3]));        // false
