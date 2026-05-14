from typing import Optional, List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def buildTree(inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
    """
    Construct binary tree from inorder and postorder traversal using index tracking.

    Key insight:
    - Postorder: left subtree, right subtree, root (last element is always root)
    - Inorder: left subtree, root, right subtree
    - Use a pointer to track the current root in postorder (traverse from end to start)
    - Find root in inorder to split into left and right subtrees

    Time Complexity: O(n²) in worst case due to linear search for root in inorder
                      O(n) if inorder is indexed (see hash_map approach)
    Space Complexity: O(h) for recursion call stack where h is height
    """
    if not inorder or not postorder:
        return None

    # Use a list to hold the postorder index as a reference (non-local variable)
    post_idx = [len(postorder) - 1]

    def build(in_start: int, in_end: int) -> Optional[TreeNode]:
        """
        Recursively build tree by processing postorder from right to left.

        Args:
            in_start, in_end: Range in inorder array
        """
        if in_start > in_end or post_idx[0] < 0:
            return None

        # Current postorder element (processing from end to start)
        root_val = postorder[post_idx[0]]
        post_idx[0] -= 1

        root = TreeNode(root_val)

        # Find root position in inorder
        root_idx = inorder.index(root_val)

        # Build right subtree first (postorder: left, right, root)
        # Since we traverse postorder backwards, right comes before left
        root.right = build(root_idx + 1, in_end)

        # Build left subtree
        root.left = build(in_start, root_idx - 1)

        return root

    return build(0, len(inorder) - 1)


# Test cases
if __name__ == "__main__":
    # Example 1: [3,9,20,null,null,15,7]
    #     3
    #    / \
    #   9  20
    #      / \
    #     15  7
    inorder1 = [9, 3, 15, 20, 7]
    postorder1 = [9, 15, 7, 20, 3]
    root1 = buildTree(inorder1, postorder1)

    def inorder_traversal(node):
        if not node:
            return []
        return inorder_traversal(node.left) + [node.val] + inorder_traversal(node.right)

    def postorder_traversal(node):
        if not node:
            return []
        return postorder_traversal(node.left) + postorder_traversal(node.right) + [node.val]

    print(inorder_traversal(root1))    # Expected: [9, 3, 15, 20, 7]
    print(postorder_traversal(root1))  # Expected: [9, 15, 7, 20, 3]

    # Example 2: Single node
    inorder2 = [1]
    postorder2 = [1]
    root2 = buildTree(inorder2, postorder2)
    print(inorder_traversal(root2))    # Expected: [1]

    # Example 3: Left skewed tree
    inorder3 = [3, 2, 1]
    postorder3 = [3, 2, 1]
    root3 = buildTree(inorder3, postorder3)
    print(inorder_traversal(root3))    # Expected: [3, 2, 1]
