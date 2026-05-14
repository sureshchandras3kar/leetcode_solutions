from typing import Optional
from collections import deque


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def sumNumbers(root: Optional[TreeNode]) -> int:
    """
    Sum all root-to-leaf numbers using iterative BFS.
    Queue stores (node, current_number) pairs.
    Time: O(n), Space: O(w) where w is max width
    """
    if not root:
        return 0

    queue = deque([(root, root.val)])
    total = 0

    while queue:
        node, current_sum = queue.popleft()

        # Leaf node: add to total
        if not node.left and not node.right:
            total += current_sum
            continue

        if node.left:
            queue.append((node.left, current_sum * 10 + node.left.val))
        if node.right:
            queue.append((node.right, current_sum * 10 + node.right.val))

    return total


# Test cases
if __name__ == "__main__":
    # Example 1: [1,2,3]
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)

    print(sumNumbers(root))  # 25 (12 + 13)

    # Example 2: [4,9,0,5,1]
    root2 = TreeNode(4)
    root2.left = TreeNode(9)
    root2.right = TreeNode(0)
    root2.left.left = TreeNode(5)
    root2.left.right = TreeNode(1)

    print(sumNumbers(root2))  # 1026 (495 + 491 + 40)

    # Example 3: Single node
    single = TreeNode(0)
    print(sumNumbers(single))  # 0
