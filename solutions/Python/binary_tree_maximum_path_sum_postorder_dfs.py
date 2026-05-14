from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def maxPathSum(root: Optional[TreeNode]) -> int:
    """
    Find maximum path sum in binary tree using post-order DFS.
    A path can pass through any node (not just root to leaf).
    Time: O(n), Space: O(h) for recursion stack
    """
    max_sum = [float('-inf')]

    def dfs(node):
        if not node:
            return 0

        # Max gain from left and right subtrees (at least 0 if negative)
        left_gain = max(0, dfs(node.left))
        right_gain = max(0, dfs(node.right))

        # Max path through this node (may bend at this node)
        max_path_through_node = node.val + left_gain + right_gain
        max_sum[0] = max(max_sum[0], max_path_through_node)

        # Return max path extending downward from this node
        return node.val + max(left_gain, right_gain)

    dfs(root)
    return max_sum[0]


# Test cases
if __name__ == "__main__":
    # Example 1: [1,2,3]
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)

    print(maxPathSum(root))  # 6 (path: 2 -> 1 -> 3)

    # Example 2: [-10,9,20,null,null,15,7]
    root2 = TreeNode(-10)
    root2.left = TreeNode(9)
    root2.right = TreeNode(20)
    root2.right.left = TreeNode(15)
    root2.right.right = TreeNode(7)

    print(maxPathSum(root2))  # 42 (path: 15 -> 20 -> 7)

    # Example 3: Single negative node
    single = TreeNode(-3)
    print(maxPathSum(single))  # -3
