/**
 * DFS approach - recursively explore each island.
 * Time: O(m * n) - visit each cell once
 * Space: O(m * n) - recursion call stack
 */
function numIslandsDFS(grid) {
    if (!grid || grid.length === 0) return 0;

    const rows = grid.length;
    const cols = grid[0].length;
    let count = 0;

    const dfs = (r, c) => {
        if (r < 0 || r >= rows || c < 0 || c >= cols || grid[r][c] === '0')
            return;

        grid[r][c] = '0';  // Mark as visited

        // Explore 4 directions
        dfs(r + 1, c);
        dfs(r - 1, c);
        dfs(r, c + 1);
        dfs(r, c - 1);
    };

    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (grid[i][j] === '1') {
                dfs(i, j);
                count++;
            }
        }
    }

    return count;
}

// Test cases
const grid1 = [["1","1","1","1","0"],
               ["1","1","0","1","0"],
               ["1","1","0","0","0"],
               ["0","0","0","0","0"]];
console.log(numIslandsDFS(grid1));  // 1

const grid2 = [["1","1","0","0","0"],
               ["1","1","0","0","0"],
               ["0","0","1","0","0"],
               ["0","0","0","1","1"]];
console.log(numIslandsDFS(grid2));  // 3
