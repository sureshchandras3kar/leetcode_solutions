/**
 * DP Top-Down (Memoization) approach for Triangle problem.
 * Starts from the top and explores paths downward.
 * Time: O(n²), Space: O(n²) for memoization
 */
function minimumTotal(triangle) {
    const memo = {};

    function dfs(row, col) {
        // Base case: at the bottom
        if (row === triangle.length - 1) {
            return triangle[row][col];
        }

        // Check memo
        const key = `${row},${col}`;
        if (key in memo) {
            return memo[key];
        }

        // Current value + minimum of two possible next positions
        const result = triangle[row][col] + Math.min(
            dfs(row + 1, col),
            dfs(row + 1, col + 1)
        );
        memo[key] = result;
        return result;
    }

    return dfs(0, 0);
}

console.log(minimumTotal([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]));  // 11
