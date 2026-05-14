/**
 * Greedy approach: track the farthest reachable index and jump count.
 *
 * Intuition: We maintain the range [currentEnd] that can be reached
 * with the current number of jumps. When we exhaust this range, we increment jumps
 * and expand to [currentEnd + 1, farthest], which is reachable with one more jump.
 *
 * Time Complexity: O(n) - single pass through array
 * Space Complexity: O(1) - only use constant extra space
 *
 * @param {number[]} nums - The array of jump values
 * @return {number} - Minimum number of jumps to reach the last index
 */
function jumpGameIIGreedy(nums) {
    if (nums.length <= 1) {
        return 0;
    }

    let jumps = 0;
    let currentEnd = 0;   // End of range reachable with current number of jumps
    let farthest = 0;     // Farthest index reachable so far

    for (let i = 0; i < nums.length - 1; i++) {
        // Update the farthest index we can reach
        farthest = Math.max(farthest, i + nums[i]);

        // If we've reached the end of current jump range, we must jump
        if (i === currentEnd) {
            jumps++;
            currentEnd = farthest;

            // Optimization: if we can reach the end, no need to continue
            if (currentEnd >= nums.length - 1) {
                break;
            }
        }
    }

    return jumps;
}

// Test cases
console.log(jumpGameIIGreedy([2, 3, 1, 1, 4]));        // 2
console.log(jumpGameIIGreedy([2, 3, 0, 6, 9, 1, 2]));  // 3
console.log(jumpGameIIGreedy([10]));                   // 0
console.log(jumpGameIIGreedy([1, 1, 1, 0]));           // 3

export default jumpGameIIGreedy;
