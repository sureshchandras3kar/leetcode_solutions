from typing import Optional, List, Dict


class TreeNode:
    def __init__(self, val: int = 0, left: Optional['TreeNode'] = None, right: Optional['TreeNode'] = None):
        self.val = val
        self.left = left
        self.right = right


def buildTree_hash_map(preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
    """
    Construct a binary tree from preorder and inorder traversal using hash map.
    Time: O(n), Space: O(n)
    """
    if not preorder or not inorder:
        return None

    # Create a map for quick lookup of indices in inorder
    inorder_map: Dict[int, int] = {val: i for i, val in enumerate(inorder)}

    def build(preorder_start: int, preorder_end: int, inorder_start: int, inorder_end: int) -> Optional[TreeNode]:
        if preorder_start > preorder_end or inorder_start > inorder_end:
            return None

        # Root is the first element in preorder
        root_val = preorder[preorder_start]
        root = TreeNode(root_val)

        # Find root position in inorder
        root_inorder_idx = inorder_map[root_val]

        # Number of elements in left subtree
        left_size = root_inorder_idx - inorder_start

        # Build left subtree from remaining preorder and inorder
        root.left = build(
            preorder_start + 1,
            preorder_start + left_size,
            inorder_start,
            root_inorder_idx - 1
        )

        # Build right subtree
        root.right = build(
            preorder_start + left_size + 1,
            preorder_end,
            root_inorder_idx + 1,
            inorder_end
        )

        return root

    return build(0, len(preorder) - 1, 0, len(inorder) - 1)


# Test cases
if __name__ == "__main__":
    # Test 1: [3,9,20,15,7], [9,3,15,20,7]
    preorder1 = [3, 9, 20, 15, 7]
    inorder1 = [9, 3, 15, 20, 7]
    root1 = buildTree_hash_map(preorder1, inorder1)
    print(f"Test 1 - Root: {root1.val}")  # 3

    # Test 2: [1], [1]
    preorder2 = [1]
    inorder2 = [1]
    root2 = buildTree_hash_map(preorder2, inorder2)
    print(f"Test 2 - Root: {root2.val}")  # 1

    # Test 3: [1,2,3], [1,3,2]
    preorder3 = [1, 2, 3]
    inorder3 = [1, 3, 2]
    root3 = buildTree_hash_map(preorder3, inorder3)
    print(f"Test 3 - Root: {root3.val}, Left: {root3.left.val}, Right: {root3.right.val}")  # 1, 2, 3
