from collections import deque

def num_islands_bfs(grid: list[list[str]]) -> int:
    """
    BFS approach - use queue to explore each island level by level.
    Time: O(m * n) - visit each cell once
    Space: O(m * n) - queue storage
    """
    if not grid or not grid[0]:
        return 0

    rows, cols = len(grid), len(grid[0])
    count = 0

    def bfs(start_r: int, start_c: int) -> None:
        """Mark all connected land cells as visited using BFS."""
        queue = deque([(start_r, start_c)])
        grid[start_r][start_c] = '0'  # Mark as visited

        while queue:
            r, c = queue.popleft()

            # Explore all 4 directions
            for dr, dc in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
                nr, nc = r + dr, c + dc
                if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == '1':
                    grid[nr][nc] = '0'  # Mark as visited
                    queue.append((nr, nc))

    # Find and count islands
    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == '1':
                bfs(i, j)
                count += 1

    return count


# Test cases
grid1 = [["1","1","1","1","0"],
         ["1","1","0","1","0"],
         ["1","1","0","0","0"],
         ["0","0","0","0","0"]]
print(num_islands_bfs(grid1))  # 1

grid2 = [["1","1","0","0","0"],
         ["1","1","0","0","0"],
         ["0","0","1","0","0"],
         ["0","0","0","1","1"]]
print(num_islands_bfs(grid2))  # 3
