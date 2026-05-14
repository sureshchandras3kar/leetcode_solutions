from typing import Optional


# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


def connect(root: Optional[Node]) -> Optional[Node]:
    """
    Populates next pointers using pre-computed links without extra queue.
    Uses next pointers of parent level to traverse current level.
    Time: O(n), Space: O(1)
    """
    if not root:
        return root

    leftmost = root

    while leftmost:
        prev = None
        current = leftmost

        while current:
            if current.left:
                if prev:
                    prev.next = current.left
                prev = current.left

            if current.right:
                if prev:
                    prev.next = current.right
                prev = current.right

            current = current.next

        leftmost = leftmost.left

    return root


# Test cases
if __name__ == "__main__":
    # Example 1: Perfect binary tree
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, Node(6), Node(7)))
    result = connect(root)
    print("Example 1: Tree with next pointers connected via pre-computed links")

    # Example 2: Single node
    single = Node(1)
    result = connect(single)
    print("Example 2: Single node tree")

    # Example 3: None
    result = connect(None)
    print("Example 3: None input")
