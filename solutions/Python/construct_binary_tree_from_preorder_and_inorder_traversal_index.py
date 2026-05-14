from typing import Optional, List


class TreeNode:
    def __init__(self, val: int = 0, left: Optional['TreeNode'] = None, right: Optional['TreeNode'] = None):
        self.val = val
        self.left = left
        self.right = right


def buildTree_index(preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
    """
    Construct a binary tree from preorder and inorder traversal using index tracking.
    Time: O(n), Space: O(h) where h is height (excluding output tree)
    """
    if not preorder or not inorder:
        return None

    self.preorder_idx = 0

    def build(inorder_start: int, inorder_end: int) -> Optional[TreeNode]:
        if inorder_start > inorder_end or self.preorder_idx >= len(preorder):
            return None

        # Root is the current element in preorder
        root_val = preorder[self.preorder_idx]
        self.preorder_idx += 1
        root = TreeNode(root_val)

        # Find root position in inorder
        root_inorder_idx = inorder.index(root_val, inorder_start, inorder_end + 1)

        # Build left subtree from inorder[inorder_start : root_inorder_idx]
        root.left = build(inorder_start, root_inorder_idx - 1)

        # Build right subtree from inorder[root_inorder_idx + 1 : inorder_end + 1]
        root.right = build(root_inorder_idx + 1, inorder_end)

        return root

    return build(0, len(inorder) - 1)


# Test cases
if __name__ == "__main__":
    # Test 1: [3,9,20,15,7], [9,3,15,20,7]
    preorder1 = [3, 9, 20, 15, 7]
    inorder1 = [9, 3, 15, 20, 7]
    root1 = buildTree_index(preorder1, inorder1)
    print(f"Test 1 - Root: {root1.val}")  # 3

    # Test 2: [1], [1]
    preorder2 = [1]
    inorder2 = [1]
    root2 = buildTree_index(preorder2, inorder2)
    print(f"Test 2 - Root: {root2.val}")  # 1

    # Test 3: [1,2,3], [1,3,2]
    preorder3 = [1, 2, 3]
    inorder3 = [1, 3, 2]
    root3 = buildTree_index(preorder3, inorder3)
    print(f"Test 3 - Root: {root3.val}, Left: {root3.left.val}, Right: {root3.right.val}")  # 1, 2, 3
