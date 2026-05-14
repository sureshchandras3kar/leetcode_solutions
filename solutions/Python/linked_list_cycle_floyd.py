from typing import Optional


class ListNode:
    def __init__(self, x: int):
        self.val = x
        self.next: Optional['ListNode'] = None


def hasCycle(head: Optional[ListNode]) -> bool:
    if not head or not head.next:
        return False

    slow = head
    fast = head

    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next

        if slow == fast:
            return True

    return False


# Test cases
if __name__ == "__main__":
    # Test case 1: Cycle exists
    node1 = ListNode(3)
    node2 = ListNode(2)
    node3 = ListNode(0)
    node4 = ListNode(-4)
    node1.next = node2
    node2.next = node3
    node3.next = node4
    node4.next = node2  # Cycle
    print(f"Cycle exists: {hasCycle(node1)}")  # True

    # Test case 2: No cycle
    node5 = ListNode(1)
    node6 = ListNode(2)
    node5.next = node6
    print(f"Cycle exists: {hasCycle(node5)}")  # False

    # Test case 3: Single node, no cycle
    node7 = ListNode(1)
    print(f"Cycle exists: {hasCycle(node7)}")  # False
