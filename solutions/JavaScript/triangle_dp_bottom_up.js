/**
 * DP Bottom-Up approach for Triangle problem.
 * Builds the solution from the bottom row upward.
 * Time: O(n²), Space: O(n)
 */
function minimumTotal(triangle) {
    const n = triangle.length;
    const dp = [...triangle[n - 1]];  // Copy last row

    // Work backwards from the second-to-last row to the top
    for (let i = n - 2; i >= 0; i--) {
        for (let j = 0; j < triangle[i].length; j++) {
            dp[j] = triangle[i][j] + Math.min(dp[j], dp[j + 1]);
        }
    }

    return dp[0];
}

console.log(minimumTotal([[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]));  // 11
