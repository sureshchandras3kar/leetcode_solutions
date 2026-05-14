from typing import Optional
from collections import defaultdict

class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

def cloneGraphDFS(node: Optional['Node']) -> Optional['Node']:
    """
    DFS approach - recursively clone nodes and build adjacency.
    Time: O(n + e)
    Space: O(n)
    """
    if not node:
        return None
    
    visited = {}
    
    def dfs(original: 'Node') -> 'Node':
        if original.val in visited:
            return visited[original.val]
        
        cloned = Node(original.val)
        visited[original.val] = cloned
        
        for neighbor in original.neighbors:
            cloned.neighbors.append(dfs(neighbor))
        
        return cloned
    
    return dfs(node)
