from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def kth_smallest_inorder_dfs(root: Optional[TreeNode], k: int) -> int:
    """
    In-order DFS approach to find kth smallest element in BST.

    In-order traversal of BST yields sorted sequence.
    Count nodes as we traverse, return when we reach the kth node.

    Time: O(k) in best case, O(n) in worst case
    Space: O(h) - recursion stack, h = height
    """
    count = [0]
    result = [None]

    def inorder(node):
        if not node or result[0] is not None:
            return

        # Traverse left subtree
        inorder(node.left)

        # Process current node
        count[0] += 1
        if count[0] == k:
            result[0] = node.val
            return

        # Traverse right subtree
        inorder(node.right)

    inorder(root)
    return result[0]


# Test cases
if __name__ == "__main__":
    # Example 1: Find 1st smallest
    root1 = TreeNode(3)
    root1.left = TreeNode(1)
    root1.right = TreeNode(4)
    root1.left.right = TreeNode(2)
    print(kth_smallest_inorder_dfs(root1, 1))  # 1

    # Example 2: Find 3rd smallest
    print(kth_smallest_inorder_dfs(root1, 3))  # 2
