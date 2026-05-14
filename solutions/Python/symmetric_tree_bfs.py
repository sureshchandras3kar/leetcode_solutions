from typing import Optional
from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_symmetric_bfs(root: Optional[TreeNode]) -> bool:
    """
    Check if a binary tree is symmetric using BFS with a queue.

    Time Complexity: O(n) - visit each node once
    Space Complexity: O(w) - queue width, where w is max nodes at any level
    """
    if not root:
        return True

    queue = deque([(root.left, root.right)])

    while queue:
        left, right = queue.popleft()

        # Both nodes are None - continue (symmetric so far)
        if not left and not right:
            continue

        # One node is None or values differ - not symmetric
        if not left or not right:
            return False
        if left.val != right.val:
            return False

        # Add pairs for next level: left's left with right's right
        # and left's right with right's left (mirror pattern)
        queue.append((left.left, right.right))
        queue.append((left.right, right.left))

    return True


# Test cases
if __name__ == "__main__":
    # Example 1: Symmetric tree
    #       1
    #      / \
    #     2   2
    #    / \ / \
    #   3  4 4  3
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.right = TreeNode(2)
    root1.left.left = TreeNode(3)
    root1.left.right = TreeNode(4)
    root1.right.left = TreeNode(4)
    root1.right.right = TreeNode(3)
    print(is_symmetric_bfs(root1))  # True

    # Example 2: Not symmetric
    #       1
    #      / \
    #     2   2
    #      \   \
    #       3   3
    root2 = TreeNode(1)
    root2.left = TreeNode(2)
    root2.right = TreeNode(2)
    root2.left.right = TreeNode(3)
    root2.right.right = TreeNode(3)
    print(is_symmetric_bfs(root2))  # False

    # Example 3: Single node
    root3 = TreeNode(1)
    print(is_symmetric_bfs(root3))  # True
