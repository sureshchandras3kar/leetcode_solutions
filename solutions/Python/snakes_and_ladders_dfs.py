def snakesAndLadders(board):
    """DFS with memoization to find shortest path."""
    n = len(board)
    
    def get_coords(pos):
        pos -= 1
        row = n - 1 - pos // n
        col = pos % n if (n - 1 - row) % 2 == 0 else n - 1 - pos % n
        return row, col
    
    memo = {}
    
    def dfs(pos):
        if pos == n * n:
            return 0
        if pos in memo:
            return memo[pos]
        
        result = float('inf')
        for i in range(1, 7):
            next_pos = pos + i
            if next_pos > n * n:
                break
            
            row, col = get_coords(next_pos)
            if board[row][col] != -1:
                next_pos = board[row][col]
            
            result = min(result, 1 + dfs(next_pos))
        
        memo[pos] = result
        return result
    
    ans = dfs(1)
    return ans if ans != float('inf') else -1
