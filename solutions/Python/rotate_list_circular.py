from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def rotate_list_circular(head: Optional[ListNode], k: int) -> Optional[ListNode]:
    """
    Rotate a linked list to the right by k positions using the circular approach.

    Approach:
    1. Calculate the effective rotation: k = k % length
    2. Create a circular list by connecting the tail to the head
    3. Find the new head position: walk (length - k) steps from the original head
    4. Break the circle at the appropriate point

    Time: O(n) - single pass to find length and establish circle, walk (length - k) steps
    Space: O(1) - only using pointers
    """
    if not head or not head.next or k == 0:
        return head

    # Step 1: Find the length of the list and the tail
    length = 0
    tail = head
    while tail.next:
        length += 1
        tail = tail.next
    length += 1  # Add 1 for the tail node itself

    # Step 2: Normalize k
    k = k % length
    if k == 0:
        return head

    # Step 3: Create a circular list
    tail.next = head

    # Step 4: Find the new head position
    # After rotation by k, the new head is at position (length - k)
    # We need to walk (length - k) steps from the original head
    steps_to_walk = length - k
    current = head
    for i in range(steps_to_walk - 1):
        current = current.next

    # Step 5: Break the circle
    new_head = current.next
    current.next = None

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
    result1 = rotate_list_circular(head1, 2)
    print(linked_list_to_list(result1))  # [4, 5, 1, 2, 3]

    # Test 2: [0, 1, 2], k=4
    head2 = create_linked_list([0, 1, 2])
    result2 = rotate_list_circular(head2, 4)
    print(linked_list_to_list(result2))  # [2, 0, 1]

    # Test 3: [1], k=1
    head3 = create_linked_list([1])
    result3 = rotate_list_circular(head3, 1)
    print(linked_list_to_list(result3))  # [1]

    # Test 4: [1, 2], k=2
    head4 = create_linked_list([1, 2])
    result4 = rotate_list_circular(head4, 2)
    print(linked_list_to_list(result4))  # [1, 2]

    # Test 5: [1, 2, 3, 4, 5], k=0
    head5 = create_linked_list([1, 2, 3, 4, 5])
    result5 = rotate_list_circular(head5, 0)
    print(linked_list_to_list(result5))  # [1, 2, 3, 4, 5]
