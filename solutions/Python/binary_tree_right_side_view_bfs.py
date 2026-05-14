from typing import Optional, List
from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def rightSideView(root: Optional[TreeNode]) -> List[int]:
    """
    Return the right side view of a binary tree using BFS.

    Time Complexity: O(n) where n is the number of nodes
    Space Complexity: O(w) where w is the maximum width
    """
    if not root:
        return []

    result = []
    queue = deque([root])

    while queue:
        level_size = len(queue)

        for i in range(level_size):
            node = queue.popleft()

            # Record the rightmost (last) node at this level
            if i == level_size - 1:
                result.append(node.val)

            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)

    return result


# Test cases
if __name__ == "__main__":
    # Example 1: [1,2,3,null,5,null,4]
    #       1
    #      / \
    #     2   3
    #      \   \
    #       5   4
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.right = TreeNode(3)
    root1.left.right = TreeNode(5)
    root1.right.right = TreeNode(4)

    result1 = rightSideView(root1)
    print(f"Right side view of tree 1: {result1}")  # Expected: [1, 3, 4]

    # Example 2: [1,null,3]
    #     1
    #      \
    #       3
    root2 = TreeNode(1)
    root2.right = TreeNode(3)

    result2 = rightSideView(root2)
    print(f"Right side view of tree 2: {result2}")  # Expected: [1, 3]

    # Example 3: Empty tree
    root3 = None
    result3 = rightSideView(root3)
    print(f"Right side view of tree 3: {result3}")  # Expected: []
