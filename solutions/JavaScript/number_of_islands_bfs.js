/**
 * BFS approach - use queue to explore each island level by level.
 * Time: O(m * n) - visit each cell once
 * Space: O(m * n) - queue storage
 */
function numIslandsBFS(grid) {
    if (!grid || grid.length === 0) return 0;

    const rows = grid.length;
    const cols = grid[0].length;
    let count = 0;

    const bfs = (sr, sc) => {
        const queue = [[sr, sc]];
        grid[sr][sc] = '0';

        const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];

        while (queue.length > 0) {
            const [r, c] = queue.shift();

            for (const [dr, dc] of dirs) {
                const nr = r + dr;
                const nc = c + dc;

                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] === '1') {
                    grid[nr][nc] = '0';
                    queue.push([nr, nc]);
                }
            }
        }
    };

    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            if (grid[i][j] === '1') {
                bfs(i, j);
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
console.log(numIslandsBFS(grid1));  // 1

const grid2 = [["1","1","0","0","0"],
               ["1","1","0","0","0"],
               ["0","0","1","0","0"],
               ["0","0","0","1","1"]];
console.log(numIslandsBFS(grid2));  // 3
