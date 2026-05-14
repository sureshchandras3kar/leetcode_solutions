from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def partition_list_two_heads(head: Optional[ListNode], x: int) -> Optional[ListNode]:
    """
    Partition linked list into two groups: values < x and values >= x.
    Uses two separate list heads and merges them at the end.

    Time Complexity: O(n) where n is the number of nodes
    Space Complexity: O(1) excluding the output list
    """
    # Create dummy nodes for two lists
    smaller_dummy = ListNode(0)
    larger_dummy = ListNode(0)

    # Pointers to build the two lists
    smaller_ptr = smaller_dummy
    larger_ptr = larger_dummy

    # Partition nodes into two lists
    current = head
    while current:
        if current.val < x:
            smaller_ptr.next = current
            smaller_ptr = smaller_ptr.next
        else:
            larger_ptr.next = current
            larger_ptr = larger_ptr.next
        current = current.next

    # Close the larger list to avoid cycles
    larger_ptr.next = None

    # Connect the two lists
    smaller_ptr.next = larger_dummy.next

    return smaller_dummy.next


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


# Helper function to convert linked list to list for easy testing
def linked_list_to_list(node):
    result = []
    while node:
        result.append(node.val)
        node = node.next
    return result


# Test cases
head = create_linked_list([1, 4, 3, 2, 5, 2])
result = partition_list_two_heads(head, 3)
print(linked_list_to_list(result))  # [1, 2, 2, 4, 3, 5]

head = create_linked_list([2, 1])
result = partition_list_two_heads(head, 2)
print(linked_list_to_list(result))  # [1, 2]

head = create_linked_list([5, 3, 1, 4, 2])
result = partition_list_two_heads(head, 3)
print(linked_list_to_list(result))  # [1, 2, 5, 3, 4]
