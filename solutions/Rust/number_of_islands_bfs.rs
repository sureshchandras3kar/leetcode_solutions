use std::collections::VecDeque;

/**
 * BFS approach - use queue to explore each island level by level.
 * Time: O(m * n) - visit each cell once
 * Space: O(m * n) - queue storage
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
                    Self::bfs(grid, i, j, rows, cols);
                    count += 1;
                }
            }
        }

        count
    }

    fn bfs(grid: &mut Vec<Vec<char>>, sr: usize, sc: usize, rows: usize, cols: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((sr, sc));
        grid[sr][sc] = '0';

        let dirs = [(1, 0), (-1, 0isize), (0, 1), (0, -1)];

        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in dirs.iter() {
                let nr = if *dr >= 0 {
                    r + (*dr as usize)
                } else if r > (-*dr) as usize {
                    r - ((-*dr) as usize)
                } else {
                    continue;
                };

                let nc = if *dc >= 0 {
                    c + (*dc as usize)
                } else if c > (-*dc) as usize {
                    c - ((-*dc) as usize)
                } else {
                    continue;
                };

                if nr < rows && nc < cols && grid[nr][nc] == '1' {
                    grid[nr][nc] = '0';
                    queue.push_back((nr, nc));
                }
            }
        }
    }
}

pub struct Solution;
