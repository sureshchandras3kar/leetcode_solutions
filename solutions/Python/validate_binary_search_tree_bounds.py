from typing import Optional

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def isValidBST(root: Optional[TreeNode]) -> bool:
    def dfs(node: Optional[TreeNode], min_val: float, max_val: float) -> bool:
        if not node: return True
        if not (min_val < node.val < max_val): return False
        return dfs(node.left, min_val, node.val) and dfs(node.right, node.val, max_val)
    return dfs(root, float('-inf'), float('inf'))

if __name__ == "__main__":
    root = TreeNode(2)
    root.left, root.right = TreeNode(1), TreeNode(3)
    print(isValidBST(root))  # True
