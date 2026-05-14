from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def lowestCommonAncestor(root: Optional[TreeNode], p: TreeNode, q: TreeNode) -> TreeNode:
    """
    Find the lowest common ancestor of two nodes in a binary tree using DFS.

    Time Complexity: O(n) where n is the number of nodes
    Space Complexity: O(h) where h is the height (call stack depth)
    """
    if root is None:
        return None

    # If either p or q is the current node, it's a potential LCA
    if root == p or root == q:
        return root

    # Search in left and right subtrees
    left = lowestCommonAncestor(root.left, p, q)
    right = lowestCommonAncestor(root.right, p, q)

    # If both p and q are found in different subtrees, root is LCA
    if left is not None and right is not None:
        return root

    # If both are in one subtree, return that subtree's result
    return left if left is not None else right


# Test cases
if __name__ == "__main__":
    # Example 1: [3,5,1,6,2,0,8,null,null,7,4]
    #       3
    #      / \
    #     5   1
    #    / \ / \
    #   6  2 0  8
    #     / \
    #    7   4
    root1 = TreeNode(3)
    root1.left = TreeNode(5)
    root1.right = TreeNode(1)
    root1.left.left = TreeNode(6)
    root1.left.right = TreeNode(2)
    root1.right.left = TreeNode(0)
    root1.right.right = TreeNode(8)
    root1.left.right.left = TreeNode(7)
    root1.left.right.right = TreeNode(4)

    p1 = root1.left  # Node 5
    q1 = root1.right  # Node 1
    result1 = lowestCommonAncestor(root1, p1, q1)
    print(f"LCA of {p1.val} and {q1.val}: {result1.val}")  # Expected: 3

    # Example 2: Same tree, p=5, q=4
    p2 = root1.left  # Node 5
    q2 = root1.left.right.right  # Node 4
    result2 = lowestCommonAncestor(root1, p2, q2)
    print(f"LCA of {p2.val} and {q2.val}: {result2.val}")  # Expected: 5

    # Example 3: [1,2]
    #     1
    #      \
    #       2
    root3 = TreeNode(1)
    root3.right = TreeNode(2)

    p3 = root3  # Node 1
    q3 = root3.right  # Node 2
    result3 = lowestCommonAncestor(root3, p3, q3)
    print(f"LCA of {p3.val} and {q3.val}: {result3.val}")  # Expected: 1
