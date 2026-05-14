/**
 * Find all unique combinations that sum to target using optimized backtracking.
 * Optimization: skip candidates that are too large.
 * Time: O(N^T), Space: O(T) for recursion depth
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
function combinationSumOptimized(candidates, target) {
    candidates.sort((a, b) => a - b);  // Sort to enable pruning
    const result = [];

    function backtrack(index, current, remaining) {
        // Base case: found a valid combination
        if (remaining === 0) {
            result.push([...current]);
            return;
        }

        // Explore: try each candidate from index onwards
        for (let i = index; i < candidates.length; i++) {
            const candidate = candidates[i];

            // Pruning: if candidate > remaining, all further candidates will be too large
            if (candidate > remaining) {
                break;
            }

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

console.log(combinationSumOptimized([2, 3, 6, 7], 7));
// Output: [[2,2,3], [7]]
