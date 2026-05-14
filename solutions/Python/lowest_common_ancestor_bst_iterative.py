from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def lowest_common_ancestor_iterative(root: Optional[TreeNode], p: TreeNode, q: TreeNode) -> TreeNode:
    """
    Iterative approach to find LCA in a BST.

    Same logic as recursive but using a while loop.
    Leverage BST property to navigate toward convergence point.

    Time: O(log n) average, O(n) worst case (skewed tree)
    Space: O(1) - only using constant extra space
    """
    current = root

    while current:
        if current.val > p.val and current.val > q.val:
            # Both p and q are in left subtree
            current = current.left
        elif current.val < p.val and current.val < q.val:
            # Both p and q are in right subtree
            current = current.right
        else:
            # p and q are on different sides or one of them is current
            return current

    return root


# Test cases
if __name__ == "__main__":
    # Example 1: LCA of 2 and 8 in tree [6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]
    root = TreeNode(6)
    root.left = TreeNode(2)
    root.right = TreeNode(8)
    root.left.left = TreeNode(0)
    root.left.right = TreeNode(4)
    root.left.right.left = TreeNode(3)
    root.left.right.right = TreeNode(5)
    root.right.left = TreeNode(7)
    root.right.right = TreeNode(9)

    p = root.left
    q = root.left.right
    result = lowest_common_ancestor_iterative(root, p, q)
    print(result.val)  # 2

    # Example 2: LCA of 2 and 4
    p = root.left
    q = root.left.right
    result = lowest_common_ancestor_iterative(root, p, q)
    print(result.val)  # 2
