from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def flatten(root: Optional[TreeNode]) -> None:
    """
    Flatten binary tree to linked list using post-order DFS with previous tracking.
    Uses a list to track the previous node in-order.
    Time: O(n), Space: O(h) for recursion stack
    """
    prev = [None]

    def dfs(node):
        if not node:
            return

        # Post-order: right, left, then process node
        dfs(node.right)
        dfs(node.left)

        node.right = prev[0]
        node.left = None
        prev[0] = node

    dfs(root)


# Test cases
if __name__ == "__main__":
    # Example 1: [1,2,5,3,4,null,6]
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.left.left = TreeNode(3)
    root.left.right = TreeNode(4)
    root.right = TreeNode(5)
    root.right.right = TreeNode(6)

    flatten(root)
    current = root
    result = []
    while current:
        result.append(current.val)
        current = current.right
    print(f"Example 1: {result}")  # [1, 2, 3, 4, 5, 6]

    # Example 2: Single node
    single = TreeNode(0)
    flatten(single)
    print("Example 2: Single node tree flattened")
