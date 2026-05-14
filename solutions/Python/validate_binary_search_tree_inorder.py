from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def isValidBST(root: Optional[TreeNode]) -> bool:
    prev = [float('-inf')]
    def dfs(node: Optional[TreeNode]) -> bool:
        if not node: return True
        if not dfs(node.left): return False
        if node.val <= prev[0]: return False
        prev[0] = node.val
        return dfs(node.right)
    return dfs(root)

if __name__ == "__main__":
    root = TreeNode(2)
    root.left, root.right = TreeNode(1), TreeNode(3)
    print(isValidBST(root))  # True
