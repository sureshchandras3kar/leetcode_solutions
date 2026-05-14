from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def rotate_list_break_point(head: Optional[ListNode], k: int) -> Optional[ListNode]:
    """
    Rotate a linked list to the right by k positions using the break point approach.

    Approach:
    1. Calculate the effective rotation: k = k % length
    2. Find the break point: the node at position (length - k - 1)
    3. Perform rotation by breaking the list at the break point

    Time: O(n) - single pass to find length, single pass to find break point
    Space: O(1) - only using pointers
    """
    if not head or not head.next or k == 0:
        return head

    # Step 1: Find the length of the list
    length = 0
    current = head
    while current:
        length += 1
        current = current.next

    # Step 2: Normalize k (handle cases where k > length)
    k = k % length
    if k == 0:
        return head

    # Step 3: Find the break point (node before rotation point)
    # We need to find the (length - k - 1)th node
    current = head
    for i in range(length - k - 1):
        current = current.next

    # Step 4: Perform rotation
    # The new head is current.next
    new_head = current.next
    current.next = None
    # Find the tail of the new list and connect it back to the old head
    tail = new_head
    while tail.next:
        tail = tail.next
    tail.next = head

    return new_head


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
    # Test 1: [1, 2, 3, 4, 5], k=2
    head1 = create_linked_list([1, 2, 3, 4, 5])
    result1 = rotate_list_break_point(head1, 2)
    print(linked_list_to_list(result1))  # [4, 5, 1, 2, 3]

    # Test 2: [0, 1, 2], k=4
    head2 = create_linked_list([0, 1, 2])
    result2 = rotate_list_break_point(head2, 4)
    print(linked_list_to_list(result2))  # [2, 0, 1]

    # Test 3: [1], k=1
    head3 = create_linked_list([1])
    result3 = rotate_list_break_point(head3, 1)
    print(linked_list_to_list(result3))  # [1]

    # Test 4: [1, 2], k=2
    head4 = create_linked_list([1, 2])
    result4 = rotate_list_break_point(head4, 2)
    print(linked_list_to_list(result4))  # [1, 2]

    # Test 5: [1, 2, 3, 4, 5], k=0
    head5 = create_linked_list([1, 2, 3, 4, 5])
    result5 = rotate_list_break_point(head5, 0)
    print(linked_list_to_list(result5))  # [1, 2, 3, 4, 5]
