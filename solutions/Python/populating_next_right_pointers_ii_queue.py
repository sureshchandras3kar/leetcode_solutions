from typing import Optional
from collections import deque


# Definition for a Node.
class Node:
    def __init__(self, val: int = 0, left: 'Node' = None, right: 'Node' = None, next: 'Node' = None):
        self.val = val
        self.left = left
        self.right = right
        self.next = next


def connect(root: Optional[Node]) -> Optional[Node]:
    """
    Populates next pointers in a perfect binary tree using level-order BFS queue.
    Time: O(n), Space: O(w) where w is max width
    """
    if not root:
        return root

    queue = deque([root])

    while queue:
        level_size = len(queue)
        prev = None

        for i in range(level_size):
            node = queue.popleft()

            if prev:
                prev.next = node
            prev = node

            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)

    return root


# Test cases
if __name__ == "__main__":
    # Example 1: Perfect binary tree
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, Node(6), Node(7)))
    result = connect(root)
    print("Example 1: Tree with next pointers connected via queue")

    # Example 2: Single node
    single = Node(1)
    result = connect(single)
    print("Example 2: Single node tree")

    # Example 3: None
    result = connect(None)
    print("Example 3: None input")
