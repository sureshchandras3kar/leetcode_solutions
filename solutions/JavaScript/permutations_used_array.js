/**
 * Generate all permutations using backtracking with used array.
 * Time: O(n! * n), Space: O(n!) for result
 * @param {number[]} nums
 * @return {number[][]}
 */
function permutationsUsedArray(nums) {
    const result = [];
    const used = new Array(nums.length).fill(false);
    const current = [];

    function backtrack() {
        // Base case: we've used all numbers
        if (current.length === nums.length) {
            result.push([...current]);
            return;
        }

        for (let i = 0; i < nums.length; i++) {
            if (used[i]) continue;

            // Choose
            current.push(nums[i]);
            used[i] = true;
            // Explore
            backtrack();
            // Unchoose
            current.pop();
            used[i] = false;
        }
    }

    backtrack();
    return result;
}

console.log(permutationsUsedArray([1, 2, 3]));
// Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
