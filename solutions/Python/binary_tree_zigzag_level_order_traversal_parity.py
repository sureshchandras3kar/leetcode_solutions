from typing import Optional, List

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def zigzagLevelOrder(root: Optional[TreeNode]) -> List[List[int]]:
    result = []
    def dfs(node: Optional[TreeNode], level: int):
        if not node: return
        if level == len(result): result.append([])
        if level % 2 == 0:
            result[level].append(node.val)
        else:
            result[level].insert(0, node.val)
        dfs(node.left, level + 1)
        dfs(node.right, level + 1)
    dfs(root, 0)
    return result

if __name__ == "__main__":
    root = TreeNode(3)
    root.left, root.right = TreeNode(9), TreeNode(20)
    root.right.left, root.right.right = TreeNode(15), TreeNode(7)
    print(zigzagLevelOrder(root))
