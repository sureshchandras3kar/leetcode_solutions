from typing import Optional
from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def isSameTree(p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
    """
    Check if two binary trees are the same using BFS (level-order traversal).

    Time Complexity: O(min(m, n)) where m and n are the number of nodes in each tree
    Space Complexity: O(min(w1, w2)) where w1 and w2 are the widths of the trees
    """
    # Use a queue to store pairs of nodes to compare
    queue = deque([(p, q)])

    while queue:
        node1, node2 = queue.popleft()

        # Both are None (equal)
        if node1 is None and node2 is None:
            continue

        # One is None or values differ (not equal)
        if node1 is None or node2 is None or node1.val != node2.val:
            return False

        # Add children to queue for comparison
        queue.append((node1.left, node2.left))
        queue.append((node1.right, node2.right))

    return True


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
