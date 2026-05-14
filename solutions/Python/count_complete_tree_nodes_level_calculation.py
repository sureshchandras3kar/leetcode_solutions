from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def countNodes(root: Optional[TreeNode]) -> int:
    """
    Count nodes in complete binary tree using level calculation.
    For complete tree, if left height == right height, left is perfect.
    Time: O(log² n), Space: O(log n) for recursion
    """
    if not root:
        return 0

    # Calculate left and right heights
    left_height = 0
    left_node = root.left
    while left_node:
        left_height += 1
        left_node = left_node.left

    right_height = 0
    right_node = root.right
    while right_node:
        right_height += 1
        right_node = right_node.right

    if left_height == right_height:
        # Left subtree is perfect: 2^h - 1 nodes + root + recursively count right
        return (1 << (left_height + 1)) - 1 + countNodes(root.right)
    else:
        # Right subtree is perfect: 2^h - 1 nodes + root + recursively count left
        return (1 << (right_height + 1)) - 1 + countNodes(root.left)


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
