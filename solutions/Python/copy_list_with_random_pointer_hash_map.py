from typing import Optional


class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = x
        self.next = next
        self.random = random


def copyRandomList(head: Optional[Node]) -> Optional[Node]:
    if not head:
        return None

    # Map original nodes to their copies
    node_map = {}

    # First pass: create all copy nodes
    current = head
    while current:
        node_map[current] = Node(current.val)
        current = current.next

    # Second pass: set next and random pointers
    current = head
    while current:
        copy_node = node_map[current]
        copy_node.next = node_map[current.next] if current.next else None
        copy_node.random = node_map[current.random] if current.random else None
        current = current.next

    return node_map[head]


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
