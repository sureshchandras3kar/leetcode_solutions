from typing import List
from collections import deque

class Node:
    def __init__(self, val, isLeaf, topLeft=None, topRight=None, bottomLeft=None, bottomRight=None):
        self.val = val
        self.isLeaf = isLeaf
        self.topLeft = topLeft
        self.topRight = topRight
        self.bottomLeft = bottomLeft
        self.bottomRight = bottomRight

def construct(grid: List[List[int]]) -> 'Node':
    n = len(grid)
    queue = deque([(0, 0, n, None)])
    root = None
    
    while queue:
        top, left, size, parent = queue.popleft()
        
        all_same = True
        val = grid[top][left]
        
        for i in range(top, top + size):
            for j in range(left, left + size):
                if grid[i][j] != val:
                    all_same = False
                    break
            if not all_same:
                break
        
        node = Node(val == 1, all_same)
        
        if root is None:
            root = node
        
        if not all_same:
            half = size // 2
            queue.append((top, left, half, node))
            queue.append((top, left + half, half, node))
            queue.append((top + half, left, half, node))
            queue.append((top + half, left + half, half, node))
    
    return root

grid = [[1,1],[1,0]]
root = construct(grid)
print(root.val, root.isLeaf)
