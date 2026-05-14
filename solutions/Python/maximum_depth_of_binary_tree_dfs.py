from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def maxDepth(root: Optional[TreeNode]) -> int:
    """
    Find the maximum depth of a binary tree using DFS (recursive).

    Time Complexity: O(n) where n is the number of nodes
    Space Complexity: O(h) where h is the height (call stack depth)
    """
    if root is None:
        return 0

    return 1 + max(maxDepth(root.left), maxDepth(root.right))


# Test cases
if __name__ == "__main__":
    # Example 1: [3,9,20,null,null,15,7]
    #       3
    #      / \
    #     9  20
    #       /  \
    #      15   7
    root1 = TreeNode(3)
    root1.left = TreeNode(9)
    root1.right = TreeNode(20)
    root1.right.left = TreeNode(15)
    root1.right.right = TreeNode(7)

    print(maxDepth(root1))  # Expected: 3

    # Example 2: [1,null,2]
    #     1
    #      \
    #       2
    root2 = TreeNode(1)
    root2.right = TreeNode(2)

    print(maxDepth(root2))  # Expected: 2

    # Example 3: Empty tree
    root3 = None
    print(maxDepth(root3))  # Expected: 0
