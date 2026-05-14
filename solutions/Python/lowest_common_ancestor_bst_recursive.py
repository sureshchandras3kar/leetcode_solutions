from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def lowest_common_ancestor_recursive(root: Optional[TreeNode], p: TreeNode, q: TreeNode) -> TreeNode:
    """
    Recursive approach to find LCA in a BST.

    Exploit BST property: if both p and q are in left subtree, LCA is in left.
    If both in right subtree, LCA is in right. Otherwise, current node is LCA.

    Time: O(log n) average, O(n) worst case (skewed tree)
    Space: O(h) - recursion stack, h = height
    """
    if root.val > p.val and root.val > q.val:
        # Both p and q are in left subtree
        return lowest_common_ancestor_recursive(root.left, p, q)
    elif root.val < p.val and root.val < q.val:
        # Both p and q are in right subtree
        return lowest_common_ancestor_recursive(root.right, p, q)
    else:
        # p and q are on different sides or one of them is root
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
    result = lowest_common_ancestor_recursive(root, p, q)
    print(result.val)  # 2

    # Example 2: LCA of 2 and 4
    p = root.left
    q = root.left.right
    result = lowest_common_ancestor_recursive(root, p, q)
    print(result.val)  # 2
