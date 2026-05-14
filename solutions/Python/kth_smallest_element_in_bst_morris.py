from typing import Optional


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def kthSmallest_morris(root: Optional[TreeNode], k: int) -> int:
    """
    Morris In-Order Traversal: O(n) time, O(1) space (no recursion stack).
    Uses threading to avoid recursion.
    """
    count = 0
    current = root

    while current:
        if not current.left:
            # No left child: visit current node
            count += 1
            if count == k:
                return current.val
            current = current.right
        else:
            # Find in-order predecessor
            predecessor = current.left
            while predecessor.right and predecessor.right != current:
                predecessor = predecessor.right

            if not predecessor.right:
                # First visit: create thread back to current
                predecessor.right = current
                current = current.left
            else:
                # Second visit: remove thread, process current
                predecessor.right = None
                count += 1
                if count == k:
                    return current.val
                current = current.right

    return -1


# Test
root = TreeNode(3)
root.left = TreeNode(1)
root.right = TreeNode(4)
root.left.right = TreeNode(2)

print(kthSmallest_morris(root, 1))  # 1
print(kthSmallest_morris(root, 3))  # 2
