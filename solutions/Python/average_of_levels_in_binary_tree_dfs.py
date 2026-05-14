from typing import Optional, List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def averageOfLevels(root: Optional[TreeNode]) -> List[float]:
    """DFS with level tracking to calculate averages."""
    sums = []
    counts = []

    def dfs(node: Optional[TreeNode], level: int) -> None:
        if not node:
            return

        if level == len(sums):
            sums.append(0)
            counts.append(0)

        sums[level] += node.val
        counts[level] += 1

        dfs(node.left, level + 1)
        dfs(node.right, level + 1)

    dfs(root, 0)

    return [sums[i] / counts[i] for i in range(len(sums))]


if __name__ == "__main__":
    root1 = TreeNode(3)
    root1.left = TreeNode(9)
    root1.right = TreeNode(20)
    root1.right.left = TreeNode(15)
    root1.right.right = TreeNode(7)

    result1 = averageOfLevels(root1)
    print(f"Averages: {result1}")  # Expected: [3.0, 14.5, 7.5]
