from typing import Optional
from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def invert_tree_bfs(root: Optional[TreeNode]) -> Optional[TreeNode]:
    """
    BFS Iterative approach to invert a binary tree.
    Uses a queue to visit nodes level by level, swapping children.

    Time Complexity: O(n) - visit each node once
    Space Complexity: O(w) - w = max width of tree (nodes at widest level)
    """
    if root is None:
        return root

    queue = deque([root])

    while queue:
        node = queue.popleft()

        # Swap left and right children
        node.left, node.right = node.right, node.left

        # Add children to queue for processing
        if node.left:
            queue.append(node.left)
        if node.right:
            queue.append(node.right)

    return root


# Test case
if __name__ == "__main__":
    # Create tree:     1
    #                /   \
    #               2     3
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)

    invert_tree_bfs(root)

    # Expected:        1
    #                /   \
    #               3     2
    print(f"Root: {root.val}")  # 1
    print(f"Left: {root.left.val}, Right: {root.right.val}")  # 3, 2
