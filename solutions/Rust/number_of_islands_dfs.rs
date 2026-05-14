/**
 * DFS approach - recursively explore each island.
 * Time: O(m * n) - visit each cell once
 * Space: O(m * n) - recursion call stack
 */
impl Solution {
    pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    Self::dfs(grid, i, j, rows, cols);
                    count += 1;
                }
            }
        }

        count
    }

    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || grid[r][c] == '0' {
            return;
        }

        grid[r][c] = '0';  // Mark as visited

        // Explore 4 directions
        if r > 0 {
            Self::dfs(grid, r - 1, c, rows, cols);
        }
        Self::dfs(grid, r + 1, c, rows, cols);
        if c > 0 {
            Self::dfs(grid, r, c - 1, rows, cols);
        }
        Self::dfs(grid, r, c + 1, rows, cols);
    }
}

pub struct Solution;
