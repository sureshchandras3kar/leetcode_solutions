#include <vector>
#include <queue>
using namespace std;

class NumberOfIslandsBFS {
public:
    int numIslands(vector<vector<char>>& grid) {
        /**
         * BFS approach - use queue to explore each island level by level.
         * Time: O(m * n) - visit each cell once
         * Space: O(m * n) - queue storage
         */
        if (grid.empty() || grid[0].empty()) return 0;

        int rows = grid.size();
        int cols = grid[0].size();
        int count = 0;

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (grid[i][j] == '1') {
                    bfs(grid, i, j, rows, cols);
                    count++;
                }
            }
        }

        return count;
    }

private:
    void bfs(vector<vector<char>>& grid, int sr, int sc, int rows, int cols) {
        queue<pair<int, int>> q;
        q.push({sr, sc});
        grid[sr][sc] = '0';

        int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

        while (!q.empty()) {
            auto [r, c] = q.front();
            q.pop();

            for (auto& dir : dirs) {
                int nr = r + dir[0];
                int nc = c + dir[1];

                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] == '1') {
                    grid[nr][nc] = '0';
                    q.push({nr, nc});
                }
            }
        }
    }
};
