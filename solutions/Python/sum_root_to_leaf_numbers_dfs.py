from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def sumNumbers(root: Optional[TreeNode]) -> int:
    """
    Sum all root-to-leaf numbers using DFS.
    Build number by appending digits as we traverse down.
    Time: O(n), Space: O(h) for recursion stack
    """
    def dfs(node, current_sum):
        if not node:
            return 0

        # Build number: multiply by 10 and add current digit
        current_sum = current_sum * 10 + node.val

        # Leaf node: return the complete number
        if not node.left and not node.right:
            return current_sum

        # Recursively process children and sum
        return dfs(node.left, current_sum) + dfs(node.right, current_sum)

    return dfs(root, 0)


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
