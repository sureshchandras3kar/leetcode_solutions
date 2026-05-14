from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def get_minimum_difference_inorder_dfs(root: Optional[TreeNode]) -> int:
    """
    In-order DFS approach to find minimum absolute difference in BST.

    In a BST, in-order traversal yields sorted values.
    We track the previous value and compute differences.

    Time: O(n) - visit each node once
    Space: O(h) - recursion stack, h = height
    """
    min_diff = [float('inf')]
    prev = [None]

    def inorder(node):
        if not node:
            return

        # Traverse left subtree
        inorder(node.left)

        # Process current node
        if prev[0] is not None:
            min_diff[0] = min(min_diff[0], node.val - prev[0])
        prev[0] = node.val

        # Traverse right subtree
        inorder(node.right)

    inorder(root)
    return min_diff[0]


# Test cases
if __name__ == "__main__":
    # Example 1: BST with multiple nodes
    root1 = TreeNode(4)
    root1.left = TreeNode(2)
    root1.right = TreeNode(6)
    root1.left.left = TreeNode(1)
    root1.left.right = TreeNode(3)
    print(get_minimum_difference_inorder_dfs(root1))  # 1

    # Example 2: Single path
    root2 = TreeNode(1)
    root2.right = TreeNode(5)
    root2.right.left = TreeNode(4)
    print(get_minimum_difference_inorder_dfs(root2))  # 1
