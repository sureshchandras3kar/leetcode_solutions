from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class BSTIterator:
    """
    Binary Search Tree Iterator using stack for in-order traversal.
    Implements lazy evaluation: next() O(1) amortized, hasNext() O(1).
    Space: O(h) where h is height
    """

    def __init__(self, root: Optional[TreeNode]):
        self.stack = []
        self._push_left(root)

    def _push_left(self, node):
        """Push all left nodes onto stack."""
        while node:
            self.stack.append(node)
            node = node.left

    def next(self) -> int:
        """
        Return next smallest element.
        Time: O(1) amortized
        """
        node = self.stack.pop()
        if node.right:
            self._push_left(node.right)
        return node.val

    def hasNext(self) -> bool:
        """
        Check if there are more elements.
        Time: O(1)
        """
        return len(self.stack) > 0


# Test cases
if __name__ == "__main__":
    # Example: [7,3,15,null,null,9,20]
    root = TreeNode(7)
    root.left = TreeNode(3)
    root.right = TreeNode(15)
    root.right.left = TreeNode(9)
    root.right.right = TreeNode(20)

    iterator = BSTIterator(root)
    result = []
    while iterator.hasNext():
        result.append(iterator.next())
    print(result)  # [3, 7, 9, 15, 20]

    # Verify in-order property
    print("In-order traversal correct:", result == sorted(result))
