from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class BSTIterator:
    """
    Binary Search Tree Iterator using pre-computed ArrayList.
    Stores all in-order elements upfront.
    Space: O(n), next() O(1), hasNext() O(1)
    """

    def __init__(self, root: Optional[TreeNode]):
        self.arr = []
        self.index = 0
        self._inorder(root)

    def _inorder(self, node):
        """Pre-compute in-order traversal into array."""
        if not node:
            return
        self._inorder(node.left)
        self.arr.append(node.val)
        self._inorder(node.right)

    def next(self) -> int:
        """
        Return next smallest element.
        Time: O(1)
        """
        val = self.arr[self.index]
        self.index += 1
        return val

    def hasNext(self) -> bool:
        """
        Check if there are more elements.
        Time: O(1)
        """
        return self.index < len(self.arr)


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
