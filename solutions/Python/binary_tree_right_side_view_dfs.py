from typing import Optional, List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def rightSideView(root: Optional[TreeNode]) -> List[int]:
    """
    Return the right side view of a binary tree using DFS (right-first).

    Time Complexity: O(n) where n is the number of nodes
    Space Complexity: O(h) where h is the height (call stack depth)
    """
    result = []

    def dfs(node: Optional[TreeNode], level: int) -> None:
        if not node:
            return

        # First time visiting this level, it's the rightmost
        if level == len(result):
            result.append(node.val)

        # Visit right subtree first, then left
        dfs(node.right, level + 1)
        dfs(node.left, level + 1)

    dfs(root, 0)
    return result


# Test cases
if __name__ == "__main__":
    # Example 1: [1,2,3,null,5,null,4]
    #       1
    #      / \
    #     2   3
    #      \   \
    #       5   4
    root1 = TreeNode(1)
    root1.left = TreeNode(2)
    root1.right = TreeNode(3)
    root1.left.right = TreeNode(5)
    root1.right.right = TreeNode(4)

    result1 = rightSideView(root1)
    print(f"Right side view of tree 1: {result1}")  # Expected: [1, 3, 4]

    # Example 2: [1,null,3]
    #     1
    #      \
    #       3
    root2 = TreeNode(1)
    root2.right = TreeNode(3)

    result2 = rightSideView(root2)
    print(f"Right side view of tree 2: {result2}")  # Expected: [1, 3]

    # Example 3: Empty tree
    root3 = None
    result3 = rightSideView(root3)
    print(f"Right side view of tree 3: {result3}")  # Expected: []
