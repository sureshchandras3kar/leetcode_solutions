from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def countNodes(root: Optional[TreeNode]) -> int:
    """
    Count nodes in complete binary tree using binary search on node positions.
    Uses existence check for node at each possible position.
    Time: O(log² n), Space: O(log n) for recursion
    """
    if not root:
        return 0

    # Find height of tree
    height = 0
    node = root
    while node:
        height += 1
        node = node.left

    # Binary search on number of nodes
    # For a complete tree of height h, nodes range from 2^(h-1) to 2^h - 1
    low = 1 << (height - 1)  # 2^(h-1)
    high = (1 << height) - 1  # 2^h - 1

    def exists(pos, height, root):
        """Check if node at position pos exists (0-indexed, 1-based actual position)."""
        left, right = 0, (1 << (height - 1)) - 1

        for _ in range(height - 1):
            mid = (left + right + 1) // 2
            if pos >= mid:
                root = root.right
                left = mid
            else:
                root = root.left
                right = mid - 1

        return root is not None

    while low <= high:
        mid = (low + high + 1) // 2
        if exists(mid, height, root):
            low = mid
        else:
            high = mid - 1

    return low


# Test cases
if __name__ == "__main__":
    # Example 1: Complete tree with 5 nodes
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)
    root.left.left = TreeNode(4)
    root.left.right = TreeNode(5)

    print(f"Example 1 node count: {countNodes(root)}")  # 5

    # Example 2: Single node
    single = TreeNode(1)
    print(f"Example 2 node count: {countNodes(single)}")  # 1

    # Example 3: Empty tree
    print(f"Example 3 node count: {countNodes(None)}")  # 0
