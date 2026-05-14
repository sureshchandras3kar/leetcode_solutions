from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def isSameTree(p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
    """
    Check if two binary trees are the same using DFS (recursive).

    Time Complexity: O(min(m, n)) where m and n are the number of nodes in each tree
    Space Complexity: O(min(h1, h2)) where h1 and h2 are the heights of the trees (call stack)
    """
    # Both nodes are None (base case: equal)
    if p is None and q is None:
        return True

    # One is None, the other isn't (not equal)
    if p is None or q is None:
        return False

    # Values differ (not equal)
    if p.val != q.val:
        return False

    # Recursively check left and right subtrees
    return isSameTree(p.left, q.left) and isSameTree(p.right, q.right)


# Test cases
if __name__ == "__main__":
    # Example 1: Same trees
    #     1           1
    #    / \         / \
    #   2   3       2   3
    p1 = TreeNode(1)
    p1.left = TreeNode(2)
    p1.right = TreeNode(3)

    q1 = TreeNode(1)
    q1.left = TreeNode(2)
    q1.right = TreeNode(3)

    print(isSameTree(p1, q1))  # Expected: True

    # Example 2: Different structure
    #     1           1
    #    /             \
    #   2               2
    p2 = TreeNode(1)
    p2.left = TreeNode(2)

    q2 = TreeNode(1)
    q2.right = TreeNode(2)

    print(isSameTree(p2, q2))  # Expected: False

    # Example 3: Different values
    #     1           1
    #    / \         / \
    #   2   1       1   2
    p3 = TreeNode(1)
    p3.left = TreeNode(2)
    p3.right = TreeNode(1)

    q3 = TreeNode(1)
    q3.left = TreeNode(1)
    q3.right = TreeNode(2)

    print(isSameTree(p3, q3))  # Expected: False

    # Example 4: Both empty
    p4 = None
    q4 = None
    print(isSameTree(p4, q4))  # Expected: True
