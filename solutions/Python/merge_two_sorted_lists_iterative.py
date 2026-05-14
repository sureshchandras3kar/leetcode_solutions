from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def merge_two_sorted_lists_iterative(
    list1: Optional[ListNode], list2: Optional[ListNode]
) -> Optional[ListNode]:
    """
    Merge two sorted linked lists iteratively.

    Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
    Space Complexity: O(1) excluding the output list
    """
    # Create a dummy node to simplify the code
    dummy = ListNode(0)
    current = dummy

    # Traverse both lists and append the smaller node
    while list1 and list2:
        if list1.val <= list2.val:
            current.next = list1
            list1 = list1.next
        else:
            current.next = list2
            list2 = list2.next
        current = current.next

    # Append the remaining nodes
    if list1:
        current.next = list1
    else:
        current.next = list2

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
list1 = create_linked_list([1, 2, 4])
list2 = create_linked_list([1, 3, 4])
result = merge_two_sorted_lists_iterative(list1, list2)
print(linked_list_to_list(result))  # [1, 1, 2, 3, 4, 4]

list1 = create_linked_list([])
list2 = create_linked_list([])
result = merge_two_sorted_lists_iterative(list1, list2)
print(linked_list_to_list(result))  # []

list1 = create_linked_list([])
list2 = create_linked_list([0])
result = merge_two_sorted_lists_iterative(list1, list2)
print(linked_list_to_list(result))  # [0]
