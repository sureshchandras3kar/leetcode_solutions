from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def kthSmallest_inorder(root: Optional[TreeNode], k: int) -> int:
    """
    In-order DFS approach: traverse BST in sorted order, return k-th element.
    O(n) time (worst case), O(h) space.
    """
    result = {"count": 0, "value": None}

    def inorder(node):
        if not node or result["value"] is not None:
            return

        inorder(node.left)

        result["count"] += 1
        if result["count"] == k:
            result["value"] = node.val
            return

        inorder(node.right)

    inorder(root)
    return result["value"]


# Test
root = TreeNode(3)
root.left = TreeNode(1)
root.right = TreeNode(4)
root.left.right = TreeNode(2)

print(kthSmallest_inorder(root, 1))  # 1
print(kthSmallest_inorder(root, 3))  # 2
