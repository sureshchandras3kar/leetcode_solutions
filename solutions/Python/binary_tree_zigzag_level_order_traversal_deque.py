from typing import Optional, List
from collections import deque

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def zigzagLevelOrder(root: Optional[TreeNode]) -> List[List[int]]:
    if not root: return []
    result = []
    queue = deque([root])
    level = 0
    while queue:
        sz = len(queue)
        level_values = deque()
        for _ in range(sz):
            node = queue.popleft()
            if level % 2 == 0:
                level_values.append(node.val)
            else:
                level_values.appendleft(node.val)
            if node.left: queue.append(node.left)
            if node.right: queue.append(node.right)
        result.append(list(level_values))
        level += 1
    return result

if __name__ == "__main__":
    root = TreeNode(3)
    root.left, root.right = TreeNode(9), TreeNode(20)
    root.right.left, root.right.right = TreeNode(15), TreeNode(7)
    print(zigzagLevelOrder(root))
