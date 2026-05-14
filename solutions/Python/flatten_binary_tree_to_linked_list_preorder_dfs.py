from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def flatten(root: Optional[TreeNode]) -> None:
    """
    Flatten binary tree to linked list using pre-order DFS.
    Recursively flattens left and right subtrees, then rewires pointers.
    Time: O(n), Space: O(h) for recursion stack
    """
    if not root:
        return

    flatten(root.left)
    flatten(root.right)

    if root.left:
        # Find the rightmost node in flattened left subtree
        rightmost = root.left
        while rightmost.right:
            rightmost = rightmost.right

        # Attach right subtree to rightmost node
        rightmost.right = root.right
        # Move flattened left subtree to right
        root.right = root.left
        root.left = None


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
