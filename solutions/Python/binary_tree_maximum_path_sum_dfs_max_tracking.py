from typing import Optional


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def __init__(self):
        self.max_sum = float('-inf')

    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        """
        Find maximum path sum using DFS with class-level max tracking.
        Maintains max_sum as instance variable updated during traversal.
        Time: O(n), Space: O(h) for recursion stack
        """
        def dfs(node):
            if not node:
                return 0

            # Max single-path sum extending from this node
            left_sum = max(0, dfs(node.left))
            right_sum = max(0, dfs(node.right))

            # Path bending at this node
            path_sum = node.val + left_sum + right_sum
            self.max_sum = max(self.max_sum, path_sum)

            # Return best single path extending downward
            return node.val + max(left_sum, right_sum)

        dfs(root)
        return self.max_sum


# Test cases
if __name__ == "__main__":
    # Example 1: [1,2,3]
    root = TreeNode(1)
    root.left = TreeNode(2)
    root.right = TreeNode(3)

    sol = Solution()
    print(sol.maxPathSum(root))  # 6 (path: 2 -> 1 -> 3)

    # Example 2: [-10,9,20,null,null,15,7]
    root2 = TreeNode(-10)
    root2.left = TreeNode(9)
    root2.right = TreeNode(20)
    root2.right.left = TreeNode(15)
    root2.right.right = TreeNode(7)

    sol2 = Solution()
    print(sol2.maxPathSum(root2))  # 42 (path: 15 -> 20 -> 7)

    # Example 3: Single negative node
    single = TreeNode(-3)
    sol3 = Solution()
    print(sol3.maxPathSum(single))  # -3
