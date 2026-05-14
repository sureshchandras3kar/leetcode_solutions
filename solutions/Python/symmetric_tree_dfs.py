from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_symmetric_dfs(root: Optional[TreeNode]) -> bool:
    """
    Check if a binary tree is symmetric using DFS recursion.

    Time Complexity: O(n) - visit each node once
    Space Complexity: O(h) - recursion stack, where h is height
    """
    def is_mirror(left: Optional[TreeNode], right: Optional[TreeNode]) -> bool:
        # Both nodes are None - symmetric
        if not left and not right:
            return True

        # One node is None or values differ - not symmetric
        if not left or not right:
            return False
        if left.val != right.val:
            return False

        # Recursively check: left's left with right's right
        # and left's right with right's left (mirror pattern)
        return is_mirror(left.left, right.right) and is_mirror(left.right, right.left)

    return is_mirror(root, root)


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
    print(is_symmetric_dfs(root1))  # True

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
    print(is_symmetric_dfs(root2))  # False
