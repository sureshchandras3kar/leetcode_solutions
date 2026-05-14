from typing import Optional
from collections import deque


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def hasPathSum(root: Optional[TreeNode], targetSum: int) -> bool:
    """
    Check if tree has root-to-leaf path summing to targetSum using iterative BFS.
    Uses queue to store (node, current_sum) pairs.
    Time: O(n), Space: O(w) where w is max width
    """
    if not root:
        return False

    queue = deque([(root, root.val)])

    while queue:
        node, current_sum = queue.popleft()

        # Check leaf node
        if not node.left and not node.right and current_sum == targetSum:
            return True

        if node.left:
            queue.append((node.left, current_sum + node.left.val))
        if node.right:
            queue.append((node.right, current_sum + node.right.val))

    return False


# Test cases
if __name__ == "__main__":
    # Example 1: [5,4,8,11,null,13,4,7,2,null,1], targetSum = 22
    root = TreeNode(5)
    root.left = TreeNode(4)
    root.left.left = TreeNode(11)
    root.left.left.left = TreeNode(7)
    root.left.left.right = TreeNode(2)
    root.right = TreeNode(8)
    root.right.left = TreeNode(13)
    root.right.right = TreeNode(4)
    root.right.right.right = TreeNode(1)

    print(hasPathSum(root, 22))  # True
    print(hasPathSum(root, 20))  # False

    # Example 2: Single node
    single = TreeNode(1)
    print(hasPathSum(single, 1))  # True
    print(hasPathSum(single, 2))  # False
