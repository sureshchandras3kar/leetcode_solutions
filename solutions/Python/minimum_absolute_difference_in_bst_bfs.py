from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def get_minimum_difference_bfs(root: Optional[TreeNode]) -> int:
    """
    BFS level-order traversal to find minimum absolute difference in BST.

    Perform in-order traversal using an iterative approach with a stack,
    then compute minimum difference between consecutive values.

    Time: O(n) - visit each node once
    Space: O(h) - queue/stack space, h = height
    """
    if not root:
        return 0

    # Iterative in-order using stack
    stack = []
    current = root
    prev = None
    min_diff = float('inf')

    while stack or current:
        # Go to leftmost node
        while current:
            stack.append(current)
            current = current.left

        # Current is None, pop from stack
        current = stack.pop()

        # Process current node
        if prev is not None:
            min_diff = min(min_diff, current.val - prev)
        prev = current.val

        # Visit right subtree
        current = current.right

    return min_diff


# Test cases
if __name__ == "__main__":
    # Example 1: BST with multiple nodes
    root1 = TreeNode(4)
    root1.left = TreeNode(2)
    root1.right = TreeNode(6)
    root1.left.left = TreeNode(1)
    root1.left.right = TreeNode(3)
    print(get_minimum_difference_bfs(root1))  # 1

    # Example 2: Single path
    root2 = TreeNode(1)
    root2.right = TreeNode(5)
    root2.right.left = TreeNode(4)
    print(get_minimum_difference_bfs(root2))  # 1
