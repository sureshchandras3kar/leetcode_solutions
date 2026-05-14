from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def getMinimumDifference_inorder(root: Optional[TreeNode]) -> int:
    """
    In-order DFS approach: traverse BST in sorted order.
    O(n) time, O(h) space where h is height.
    """
    result = {"min_diff": float('inf'), "prev": None}

    def inorder(node):
        if not node:
            return

        inorder(node.left)

        # Visit node
        if result["prev"] is not None:
            result["min_diff"] = min(result["min_diff"], node.val - result["prev"])
        result["prev"] = node.val

        inorder(node.right)

    inorder(root)
    return result["min_diff"]


# Test
root = TreeNode(4)
root.left = TreeNode(2)
root.right = TreeNode(6)
root.left.left = TreeNode(1)
root.left.right = TreeNode(3)

print(getMinimumDifference_inorder(root))  # 1
