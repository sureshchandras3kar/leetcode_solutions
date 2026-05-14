from typing import Optional, List


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def buildTree(inorder: List[int], postorder: List[int]) -> Optional[TreeNode]:
    """
    Construct binary tree from inorder and postorder traversal using HashMap.

    Key insight:
    - Postorder: left subtree, right subtree, root (last element is always root)
    - Inorder: left subtree, root, right subtree

    Use a HashMap to quickly find the root's position in inorder,
    then recursively build left and right subtrees.

    Time Complexity: O(n) where n is the number of nodes (HashMap lookup is O(1))
    Space Complexity: O(n) for the HashMap and recursion call stack
    """
    if not inorder or not postorder:
        return None

    # HashMap to store inorder values and their indices for O(1) lookup
    inorder_map = {val: idx for idx, val in enumerate(inorder)}

    def build(post_start: int, post_end: int, in_start: int, in_end: int) -> Optional[TreeNode]:
        """
        Recursively build tree from postorder and inorder ranges.

        Args:
            post_start, post_end: Range in postorder array
            in_start, in_end: Range in inorder array
        """
        if post_start > post_end or in_start > in_end:
            return None

        # Last element in postorder range is the root
        root_val = postorder[post_end]
        root = TreeNode(root_val)

        # Find root position in inorder
        root_idx = inorder_map[root_val]

        # Number of nodes in left subtree
        left_size = root_idx - in_start

        # Recursively build left subtree
        # Postorder: [left subtree], [right subtree], root
        # Inorder: [left subtree], root, [right subtree]
        root.left = build(
            post_start,           # Left subtree starts at beginning of postorder
            post_start + left_size - 1,  # Left subtree ends after left_size elements
            in_start,             # Left subtree starts at beginning of inorder
            root_idx - 1          # Left subtree ends before root
        )

        root.right = build(
            post_start + left_size,  # Right subtree starts after left subtree
            post_end - 1,         # Right subtree ends before root
            root_idx + 1,         # Right subtree starts after root
            in_end                # Right subtree ends at end of inorder
        )

        return root

    return build(0, len(postorder) - 1, 0, len(inorder) - 1)


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
