package main

/**
 * DFS approach - recursively explore each island.
 * Time: O(m * n) - visit each cell once
 * Space: O(m * n) - recursion call stack
 */
func numIslandsDFS(grid [][]byte) int {
    if len(grid) == 0 || len(grid[0]) == 0 {
        return 0
    }

    rows := len(grid)
    cols := len(grid[0])
    count := 0

    var dfs func(r, c int)
    dfs = func(r, c int) {
        if r < 0 || r >= rows || c < 0 || c >= cols || grid[r][c] == '0' {
            return
        }

        grid[r][c] = '0'  // Mark as visited

        // Explore 4 directions
        dfs(r+1, c)
        dfs(r-1, c)
        dfs(r, c+1)
        dfs(r, c-1)
    }

    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            if grid[i][j] == '1' {
                dfs(i, j)
                count++
            }
        }
    }

    return count
}
