from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def reverse_k_group_recursive(head: Optional[ListNode], k: int) -> Optional[ListNode]:
    """
    Reverse nodes in k-group recursively.

    Approach: Use recursion to handle each group independently.
    - Check if there are k nodes remaining
    - Reverse the first k nodes
    - Recursively process the rest of the list
    - Connect the reversed group to the result of the recursive call

    Time: O(n) - visit each node once
    Space: O(n/k) - recursion depth is proportional to number of groups
    """

    def reverse_helper(node, k):
        """
        Reverse the first k nodes starting from 'node'.
        Returns a tuple: (new_head, tail_of_reversed_group)
        """
        if not node:
            return node, node

        # Find the k-th node
        kth = node
        for i in range(k - 1):
            if not kth.next:
                # Not enough nodes, return as is
                return node, kth
            kth = kth.next

        # Save the next group's head
        next_group_head = kth.next

        # Reverse the current group (from node to kth)
        prev = next_group_head
        curr = node

        while curr != next_group_head:
            next_temp = curr.next
            curr.next = prev
            prev = curr
            curr = next_temp

        # curr is now at node (which will be the tail after reversal)
        # prev is now at kth (which will be the head after reversal)
        # So the head of reversed group is kth

        # Recursively process the rest
        node.next, _ = reverse_helper(next_group_head, k)

        return kth, node

    result, _ = reverse_helper(head, k)
    return result


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
result1 = reverse_k_group_recursive(head1, 2)
print(linked_list_to_list(result1))  # [2, 1, 4, 3, 5]

head2 = create_linked_list([1, 2, 3, 4, 5])
result2 = reverse_k_group_recursive(head2, 3)
print(linked_list_to_list(result2))  # [3, 2, 1, 4, 5]

head3 = create_linked_list([1])
result3 = reverse_k_group_recursive(head3, 1)
print(linked_list_to_list(result3))  # [1]
