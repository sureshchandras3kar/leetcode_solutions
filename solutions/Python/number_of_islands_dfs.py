def num_islands_dfs(grid: list[list[str]]) -> int:
    """
    DFS approach - recursively explore each island from any land cell.
    Time: O(m * n) - visit each cell once
    Space: O(m * n) - recursion call stack in worst case
    """
    if not grid or not grid[0]:
        return 0

    rows, cols = len(grid), len(grid[0])
    count = 0

    def dfs(r: int, c: int) -> None:
        """Mark all connected land cells as visited."""
        if r < 0 or r >= rows or c < 0 or c >= cols or grid[r][c] == '0':
            return

        grid[r][c] = '0'  # Mark as visited

        # Explore all 4 directions
        dfs(r + 1, c)  # Down
        dfs(r - 1, c)  # Up
        dfs(r, c + 1)  # Right
        dfs(r, c - 1)  # Left

    # Find and count islands
    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == '1':
                dfs(i, j)
                count += 1

    return count


# Test cases
grid1 = [["1","1","1","1","0"],
         ["1","1","0","1","0"],
         ["1","1","0","0","0"],
         ["0","0","0","0","0"]]
print(num_islands_dfs(grid1))  # 1

grid2 = [["1","1","0","0","0"],
         ["1","1","0","0","0"],
         ["0","0","1","0","0"],
         ["0","0","0","1","1"]]
print(num_islands_dfs(grid2))  # 3
