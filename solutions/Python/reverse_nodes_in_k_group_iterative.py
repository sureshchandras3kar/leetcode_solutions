from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def reverse_k_group_iterative(head: Optional[ListNode], k: int) -> Optional[ListNode]:
    """
    Reverse nodes in k-group iteratively.

    Approach: Reverse each group of k nodes in-place using iteration.
    - Find the k-th node to determine the group boundary
    - Reverse the group from current to k-th node
    - Link the reversed group back to the previous group
    - Move to the next group and repeat

    Time: O(n) - visit each node once
    Space: O(1) - only pointers, no extra structures
    """
    # Check if there are at least k nodes to reverse
    def get_kth_node(node, k):
        """Find the k-th node starting from 'node'."""
        while node and k > 1:
            node = node.next
            k -= 1
        return node

    # Dummy node to simplify logic (no need to handle head specially)
    dummy = ListNode(0)
    dummy.next = head

    prev_group = dummy  # Tracks the node before the current group

    while True:
        # Check if there are at least k nodes remaining
        kth_node = get_kth_node(prev_group, k + 1)
        if not kth_node:
            break

        # Mark the start and end of the current group
        group_start = prev_group.next
        group_end = kth_node

        # Save the next group's start (before we reverse the current group)
        next_group_start = group_end.next

        # Reverse the current group
        # We reverse from group_start to group_end (inclusive)
        prev = next_group_start  # The node after group_end becomes the new tail
        curr = group_start

        while curr != next_group_start:
            next_temp = curr.next
            curr.next = prev
            prev = curr
            curr = next_temp

        # Link the previous group to the reversed current group
        prev_group.next = group_end
        prev_group = group_start

    return dummy.next


# Test cases
def create_linked_list(values):
    """Helper to create a linked list from a list of values."""
    if not values:
        return None
    head = ListNode(values[0])
    current = head
    for val in values[1:]:
        current.next = ListNode(val)
        current = current.next
    return head


def linked_list_to_list(head):
    """Helper to convert linked list to list for easy comparison."""
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result


# Test
head1 = create_linked_list([1, 2, 3, 4, 5])
result1 = reverse_k_group_iterative(head1, 2)
print(linked_list_to_list(result1))  # [2, 1, 4, 3, 5]

head2 = create_linked_list([1, 2, 3, 4, 5])
result2 = reverse_k_group_iterative(head2, 3)
print(linked_list_to_list(result2))  # [3, 2, 1, 4, 5]

head3 = create_linked_list([1])
result3 = reverse_k_group_iterative(head3, 1)
print(linked_list_to_list(result3))  # [1]
