from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def remove_nth_node_two_pass(head: Optional[ListNode], n: int) -> Optional[ListNode]:
    """
    Remove the nth node from the end of a linked list using two passes.

    Approach:
    1. First pass: count the total number of nodes
    2. Second pass: find and skip the nth node from the end

    Time: O(L) + O(L-n) = O(L)
    Space: O(1)
    """
    # First pass: count the total nodes
    length = 0
    current = head
    while current:
        length += 1
        current = current.next

    # Edge case: removing the head node
    if length == n:
        return head.next

    # Second pass: find the node before the one to remove
    current = head
    for i in range(length - n - 1):
        current = current.next

    # Remove the nth node
    current.next = current.next.next
    return head


# Helper function to create a linked list from a list
def create_linked_list(values):
    if not values:
        return None
    head = ListNode(values[0])
    current = head
    for val in values[1:]:
        current.next = ListNode(val)
        current = current.next
    return head


# Helper function to convert linked list to list for easy printing
def linked_list_to_list(head):
    result = []
    current = head
    while current:
        result.append(current.val)
        current = current.next
    return result


# Test cases
if __name__ == "__main__":
    # Test 1: [1, 2, 3, 4, 5], n=2
    head1 = create_linked_list([1, 2, 3, 4, 5])
    result1 = remove_nth_node_two_pass(head1, 2)
    print(linked_list_to_list(result1))  # [1, 2, 3, 5]

    # Test 2: [1], n=1
    head2 = create_linked_list([1])
    result2 = remove_nth_node_two_pass(head2, 1)
    print(linked_list_to_list(result2))  # []

    # Test 3: [1, 2], n=2
    head3 = create_linked_list([1, 2])
    result3 = remove_nth_node_two_pass(head3, 2)
    print(linked_list_to_list(result3))  # [2]

    # Test 4: [1, 2], n=1
    head4 = create_linked_list([1, 2])
    result4 = remove_nth_node_two_pass(head4, 1)
    print(linked_list_to_list(result4))  # [1]
