/**
 * Find all unique combinations that sum to target using backtracking.
 * Time: O(N^T), Space: O(T) for recursion depth
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
function combinationSumBasic(candidates, target) {
    const result = [];

    function backtrack(index, current, remaining) {
        // Base case: found a valid combination
        if (remaining === 0) {
            result.push([...current]);
            return;
        }

        // Base case: no valid combinations possible
        if (remaining < 0) {
            return;
        }

        // Explore: try each candidate from index onwards
        for (let i = index; i < candidates.length; i++) {
            const candidate = candidates[i];
            // Choose
            current.push(candidate);
            // Explore: can reuse the same candidate (i, not i+1)
            backtrack(i, current, remaining - candidate);
            // Unchoose
            current.pop();
        }
    }

    backtrack(0, [], target);
    return result;
}

console.log(combinationSumBasic([2, 3, 6, 7], 7));
// Output: [[2,2,3], [7]]
