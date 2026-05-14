from typing import Optional
from collections import deque

class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

def cloneGraphBFS(node: Optional['Node']) -> Optional['Node']:
    """
    BFS approach - iterate through nodes using queue.
    Time: O(n + e)
    Space: O(n)
    """
    if not node:
        return None
    
    visited = {}
    queue = deque([node])
    visited[node.val] = Node(node.val)
    
    while queue:
        original = queue.popleft()
        cloned = visited[original.val]
        
        for neighbor in original.neighbors:
            if neighbor.val not in visited:
                visited[neighbor.val] = Node(neighbor.val)
                queue.append(neighbor)
            cloned.neighbors.append(visited[neighbor.val])
    
    return visited[node.val]
