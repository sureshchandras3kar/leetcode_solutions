from typing import Optional, List
from collections import deque


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def sortedArrayToBST(nums: List[int]) -> Optional[TreeNode]:
    if not nums:
        return None

    root = TreeNode()
    queue = deque([(root, 0, len(nums) - 1)])

    while queue:
        node, left, right = queue.popleft()

        mid = (left + right) // 2
        node.val = nums[mid]

        if left <= mid - 1:
            node.left = TreeNode()
            queue.append((node.left, left, mid - 1))

        if mid + 1 <= right:
            node.right = TreeNode()
            queue.append((node.right, mid + 1, right))

    return root


# Example usage
nums = [-10, -3, 0, 5, 9]
root = sortedArrayToBST(nums)
print(root.val)  # 0
print(root.left.val)  # -3
print(root.right.val)  # 5
