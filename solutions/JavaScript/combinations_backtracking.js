/**
 * Generate all combinations of k numbers from 1 to n using backtracking.
 * Time: O(C(n,k) * k), Space: O(C(n,k) * k) for result
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
function combinationsBacktracking(n, k) {
    const result = [];

    function backtrack(start, current) {
        // Base case: we've selected k numbers
        if (current.length === k) {
            result.push([...current]);
            return;
        }

        // Explore: try each remaining number
        for (let i = start; i <= n; i++) {
            // Choose
            current.push(i);
            // Explore
            backtrack(i + 1, current);
            // Unchoose
            current.pop();
        }
    }

    backtrack(1, []);
    return result;
}

console.log(combinationsBacktracking(4, 2));
// Output: [[1,2], [1,3], [1,4], [2,3], [2,4], [3,4]]
