package main

/**
 * BFS approach - use queue to explore each island level by level.
 * Time: O(m * n) - visit each cell once
 * Space: O(m * n) - queue storage
 */
func numIslandsBFS(grid [][]byte) int {
    if len(grid) == 0 || len(grid[0]) == 0 {
        return 0
    }

    rows := len(grid)
    cols := len(grid[0])
    count := 0

    bfs := func(sr, sc int) {
        queue := [][2]int{{sr, sc}}
        grid[sr][sc] = '0'

        dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

        for len(queue) > 0 {
            r, c := queue[0][0], queue[0][1]
            queue = queue[1:]

            for _, dir := range dirs {
                nr := r + dir[0]
                nc := c + dir[1]

                if nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] == '1' {
                    grid[nr][nc] = '0'
                    queue = append(queue, [2]int{nr, nc})
                }
            }
        }
    }

    for i := 0; i < rows; i++ {
        for j := 0; j < cols; j++ {
            if grid[i][j] == '1' {
                bfs(i, j)
                count++
            }
        }
    }

    return count
}
