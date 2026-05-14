from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def partition_list_single_rearrange(head: Optional[ListNode], x: int) -> Optional[ListNode]:
    """
    Partition linked list into two groups: values < x and values >= x.
    Rearranges nodes in a single pass without creating separate lists.

    Time Complexity: O(n) where n is the number of nodes
    Space Complexity: O(1) excluding the output list
    """
    # Create a dummy node to simplify edge cases
    dummy = ListNode(0)
    dummy.next = head

    # Find the node just before the partition point
    prev = dummy
    current = head

    while current and current.val < x:
        prev = current
        current = current.next

    # Now prev is the last node with value < x (or dummy if all nodes >= x)
    # current is the first node with value >= x (or None if all nodes < x)

    # If all nodes are less than x or list is empty, return as is
    if not current:
        return dummy.next

    # Now we need to insert nodes with value < x before the partition point
    partition_prev = prev  # Last node of < x group
    run = current  # Start from the partition point

    while run:
        if run.val < x:
            # Remove run from its position
            current.next = run.next

            # Insert run before the partition point
            run.next = partition_prev.next
            partition_prev.next = run

            # Move partition_prev forward
            partition_prev = run

            # Current stays the same to check the next node
            run = current.next
        else:
            # Move forward
            current = run
            run = run.next

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


# Helper function to convert linked list to list for easy testing
def linked_list_to_list(node):
    result = []
    while node:
        result.append(node.val)
        node = node.next
    return result


# Test cases
head = create_linked_list([1, 4, 3, 2, 5, 2])
result = partition_list_single_rearrange(head, 3)
print(linked_list_to_list(result))  # [1, 2, 2, 4, 3, 5]

head = create_linked_list([2, 1])
result = partition_list_single_rearrange(head, 2)
print(linked_list_to_list(result))  # [1, 2]

head = create_linked_list([5, 3, 1, 4, 2])
result = partition_list_single_rearrange(head, 3)
print(linked_list_to_list(result))  # [1, 2, 5, 3, 4]
