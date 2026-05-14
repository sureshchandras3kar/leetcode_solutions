from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def remove_nth_node_two_pointers(head: Optional[ListNode], n: int) -> Optional[ListNode]:
    """
    Remove the nth node from the end of a linked list using two pointers in one pass.

    Approach:
    1. Create a dummy node pointing to head (handles edge case of removing head)
    2. Use fast and slow pointers with a gap of n nodes between them
    3. Move both pointers until fast reaches the end
    4. Skip the target node by adjusting the slow pointer

    Time: O(L) single pass
    Space: O(1)
    """
    # Create a dummy node to handle edge case of removing the head
    dummy = ListNode(0)
    dummy.next = head
    slow = dummy
    fast = dummy

    # Move fast pointer n+1 steps ahead
    for i in range(n + 1):
        if not fast:
            return head
        fast = fast.next

    # Move both pointers until fast reaches the end
    while fast:
        slow = slow.next
        fast = fast.next

    # Remove the nth node
    slow.next = slow.next.next
    return dummy.next


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
    result1 = remove_nth_node_two_pointers(head1, 2)
    print(linked_list_to_list(result1))  # [1, 2, 3, 5]

    # Test 2: [1], n=1
    head2 = create_linked_list([1])
    result2 = remove_nth_node_two_pointers(head2, 1)
    print(linked_list_to_list(result2))  # []

    # Test 3: [1, 2], n=2
    head3 = create_linked_list([1, 2])
    result3 = remove_nth_node_two_pointers(head3, 2)
    print(linked_list_to_list(result3))  # [2]

    # Test 4: [1, 2], n=1
    head4 = create_linked_list([1, 2])
    result4 = remove_nth_node_two_pointers(head4, 1)
    print(linked_list_to_list(result4))  # [1]
