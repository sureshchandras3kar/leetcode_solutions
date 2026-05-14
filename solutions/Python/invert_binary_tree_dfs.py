from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def invert_tree_dfs(root: Optional[TreeNode]) -> Optional[TreeNode]:
    """
    DFS Recursive approach to invert a binary tree.
    Recursively swap left and right children for each node.

    Time Complexity: O(n) - visit each node once
    Space Complexity: O(h) - recursion stack depth (h = height)
    """
    if root is None:
        return None

    # Swap left and right children
    root.left, root.right = root.right, root.left

    # Recursively invert left and right subtrees
    invert_tree_dfs(root.left)
    invert_tree_dfs(root.right)

    return root


# Test case
if __name__ == "__main__":
    # Create tree:     1
    #                /   \
    #               2     3
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)

    invert_tree_dfs(root)

    # Expected:        1
    #                /   \
    #               3     2
    print(f"Root: {root.val}")  # 1
    print(f"Left: {root.left.val}, Right: {root.right.val}")  # 3, 2
