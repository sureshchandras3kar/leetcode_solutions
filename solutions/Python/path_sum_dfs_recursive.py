from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def hasPathSum(root: Optional[TreeNode], targetSum: int) -> bool:
    """
    Check if tree has root-to-leaf path summing to targetSum using recursive DFS.
    Time: O(n), Space: O(h) for recursion stack
    """
    if not root:
        return False

    # Leaf node check
    if not root.left and not root.right:
        return root.val == targetSum

    # Subtract current value and check left and right subtrees
    remaining = targetSum - root.val
    return hasPathSum(root.left, remaining) or hasPathSum(root.right, remaining)


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
