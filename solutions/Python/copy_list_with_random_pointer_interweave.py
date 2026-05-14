from typing import Optional


class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = x
        self.next = next
        self.random = random


def copyRandomList(head: Optional[Node]) -> Optional[Node]:
    if not head:
        return None

    # First pass: create copies and interleave them
    # original -> copy -> original -> copy -> ...
    current = head
    while current:
        copy = Node(current.val)
        copy.next = current.next
        current.next = copy
        current = copy.next

    # Second pass: set random pointers for copy nodes
    current = head
    while current:
        if current.random:
            current.next.random = current.random.next
        current = current.next.next

    # Third pass: restore original list and extract copy list
    current = head
    copy_head = head.next
    while current:
        copy = current.next
        current.next = copy.next
        copy.next = copy.next.next if copy.next else None
        current = current.next

    return copy_head


# Test cases
def print_list(head: Optional[Node]):
    result = []
    current = head
    while current:
        result.append((current.val, current.random.val if current.random else None))
        current = current.next
    return result


# Example 1: [[7,None],[13,0],[11,4],[10,2],[1,0]]
node1 = Node(7)
node2 = Node(13)
node3 = Node(11)
node4 = Node(10)
node5 = Node(1)

node1.next = node2
node2.next = node3
node3.next = node4
node4.next = node5

node1.random = None
node2.random = node1
node3.random = node5
node4.random = node3
node5.random = node1

copy = copyRandomList(node1)
print(print_list(copy))  # [(7, None), (13, 7), (11, 1), (10, 11), (1, 7)]

# Example 2: [[1,1],[2,1]]
node6 = Node(1)
node7 = Node(2)
node6.next = node7
node6.random = node6
node7.random = node6

copy2 = copyRandomList(node6)
print(print_list(copy2))  # [(1, 1), (2, 1)]
