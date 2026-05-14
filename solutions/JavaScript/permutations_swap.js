/**
 * Generate all permutations using backtracking with swapping approach.
 * Time: O(n! * n), Space: O(n!) for result
 * @param {number[]} nums
 * @return {number[][]}
 */
function permutationsSwap(nums) {
    const result = [];

    function backtrack(first) {
        // Base case: all elements are placed
        if (first === nums.length) {
            result.push([...nums]);
            return;
        }

        for (let i = first; i < nums.length; i++) {
            // Swap
            [nums[first], nums[i]] = [nums[i], nums[first]];
            // Backtrack
            backtrack(first + 1);
            // Swap back
            [nums[first], nums[i]] = [nums[i], nums[first]];
        }
    }

    backtrack(0);
    return result;
}

console.log(permutationsSwap([1, 2, 3]));
// Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
