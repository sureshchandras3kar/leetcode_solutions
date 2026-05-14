from collections import deque

def snakesAndLadders(board):
    """BFS to find shortest path in game board."""
    n = len(board)
    
    def get_coords(pos):
        pos -= 1
        row = n - 1 - pos // n
        col = pos % n if (n - 1 - row) % 2 == 0 else n - 1 - pos % n
        return row, col
    
    queue = deque([(1, 0)])
    visited = {1}
    
    while queue:
        pos, moves = queue.popleft()
        if pos == n * n:
            return moves
        
        for i in range(1, 7):
            next_pos = pos + i
            if next_pos > n * n:
                break
            
            row, col = get_coords(next_pos)
            if board[row][col] != -1:
                next_pos = board[row][col]
            
            if next_pos not in visited:
                visited.add(next_pos)
                queue.append((next_pos, moves + 1))
    
    return -1
