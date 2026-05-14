from typing import List

class Node:
    def __init__(self, val, isLeaf, topLeft=None, topRight=None, bottomLeft=None, bottomRight=None):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight

def construct(grid: List[List[int]]) -> 'Node':
    def dfs(top, left, size):
        all_same = True
        val = grid[top][left]
        
        for i in range(top, top + size):
            for j in range(left, left + size):
                if grid[i][j] != val:
                    all_same = False
                    break
            if not all_same:
                break
        
        if all_same:
            return Node(val == 1, True)
        
        half = size // 2
        topLeft = dfs(top, left, half)
        topRight = dfs(top, left + half, half)
        bottomLeft = dfs(top + half, left, half)
        bottomRight = dfs(top + half, left + half, half)
        
        return Node(1, False, topLeft, topRight, bottomLeft, bottomRight)
    
    return dfs(0, 0, len(grid))

grid = [[1,1],[1,0]]
root = construct(grid)
print(root.val, root.isLeaf)
