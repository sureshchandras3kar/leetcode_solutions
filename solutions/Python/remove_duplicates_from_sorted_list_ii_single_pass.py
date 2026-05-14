from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def deleteDuplicates(head: Optional[ListNode]) -> Optional[ListNode]:
    """
    Remove all nodes with duplicate values from sorted linked list in single pass.

    Approach: Use a dummy node and prev pointer. When we find duplicates, skip all nodes
    with that value by advancing current and updating prev.next to point past the group.
    Time: O(n) — single pass through the list
    Space: O(1) — only pointer variables
    """
    if not head:
        return None

    # Create dummy node to handle edge case where head is duplicate
    dummy = ListNode(0)
    dummy.next = head
    prev = dummy
    current = head

    while current:
        # Check if current node is the start of a duplicate group
        if current.next and current.val == current.next.val:
            # Skip all nodes with the same value
            value = current.val
            while current and current.val == value:
                current = current.next
            # Link prev to the first non-duplicate node
            prev.next = current
        else:
            # Current node is unique, keep it
            prev = current
            current = current.next

    return dummy.next


# Test cases
if __name__ == "__main__":
    # Helper function to create linked list from array
    def create_list(arr):
        if not arr:
            return None
        head = ListNode(arr[0])
        current = head
        for val in arr[1:]:
            current.next = ListNode(val)
            current = current.next
        return head

    # Helper function to convert linked list to array
    def list_to_array(head):
        result = []
        current = head
        while current:
            result.append(current.val)
            current = current.next
        return result

    # Test case 1: [1,2,3,3,4,4,5]
    head1 = create_list([1, 2, 3, 3, 4, 4, 5])
    result1 = deleteDuplicates(head1)
    print(list_to_array(result1))  # [1, 2, 5]

    # Test case 2: [1,1,1,2,3]
    head2 = create_list([1, 1, 1, 2, 3])
    result2 = deleteDuplicates(head2)
    print(list_to_array(result2))  # [2, 3]

    # Test case 3: [1,1]
    head3 = create_list([1, 1])
    result3 = deleteDuplicates(head3)
    print(list_to_array(result3))  # []
