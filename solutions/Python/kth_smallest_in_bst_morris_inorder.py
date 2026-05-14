from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def kth_smallest_morris_inorder(root: Optional[TreeNode], k: int) -> int:
    """
    Morris In-Order traversal to find kth smallest element in BST.

    Uses tree threading to traverse without recursion or extra stack space.
    Space-efficient: O(1) extra space (modifying and restoring tree structure).

    Time: O(n) - visit each node twice
    Space: O(1) - no extra data structures
    """
    count = 0
    current = root

    while current:
        if current.left is None:
            # Left is None, process current node
            count += 1
            if count == k:
                return current.val
            current = current.right
        else:
            # Find in-order predecessor
            predecessor = current.left
            while predecessor.right and predecessor.right != current:
                predecessor = predecessor.right

            if predecessor.right is None:
                # Create link to current node
                predecessor.right = current
                current = current.left
            else:
                # Link exists, remove it and process current
                predecessor.right = None
                count += 1
                if count == k:
                    return current.val
                current = current.right

    return -1


# Test cases
if __name__ == "__main__":
    # Example 1: Find 1st smallest
    root1 = TreeNode(3)
    root1.left = TreeNode(1)
    root1.right = TreeNode(4)
    root1.left.right = TreeNode(2)
    print(kth_smallest_morris_inorder(root1, 1))  # 1

    # Example 2: Find 3rd smallest
    print(kth_smallest_morris_inorder(root1, 3))  # 2
