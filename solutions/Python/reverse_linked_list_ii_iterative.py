from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def reverseBetween(head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
    """
    Reverse a portion of a linked list from position left to right (1-indexed).

    Time Complexity: O(n) - single pass through the list
    Space Complexity: O(1) - only pointer variables

    Strategy:
    1. Create a dummy node to handle edge case of reversing from head
    2. Advance prev pointer to position left-1
    3. Perform (right - left) reversals by moving nodes to the front of the sublist
    4. Return dummy.next
    """
    if left == right:
        return head

    # Create dummy node pointing to head
    dummy = ListNode(0, head)
    prev = dummy

    # Advance prev to position left-1
    for _ in range(left - 1):
        prev = prev.next

    # curr is the first node to reverse (at position left)
    curr = prev.next

    # Perform (right - left) reversals
    for _ in range(right - left):
        # next_temp is the node we want to move to the front
        next_temp = curr.next
        # Bypass next_temp by connecting curr to next_temp.next
        curr.next = next_temp.next
        # Move next_temp to the front of the sublist
        next_temp.next = prev.next
        prev.next = next_temp

    return dummy.next


# Test cases
def list_to_array(head: Optional[ListNode]) -> list:
    """Convert linked list to array for easy comparison."""
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result


def array_to_list(arr: list) -> Optional[ListNode]:
    """Convert array to linked list."""
    if not arr:
        return None
    head = ListNode(arr[0])
    current = head
    for val in arr[1:]:
        current.next = ListNode(val)
        current = current.next
    return head


# Test 1: Reverse middle portion
head1 = array_to_list([1, 2, 3, 4, 5])
result1 = reverseBetween(head1, 2, 4)
print(list_to_array(result1))  # [1, 4, 3, 2, 5]

# Test 2: Single node (no reversal)
head2 = array_to_list([5])
result2 = reverseBetween(head2, 1, 1)
print(list_to_array(result2))  # [5]

# Test 3: Reverse entire list
head3 = array_to_list([1, 2])
result3 = reverseBetween(head3, 1, 2)
print(list_to_array(result3))  # [2, 1]

# Test 4: Reverse from start
head4 = array_to_list([1, 2, 3, 4, 5])
result4 = reverseBetween(head4, 1, 3)
print(list_to_array(result4))  # [3, 2, 1, 4, 5]

# Test 5: Reverse at end
head5 = array_to_list([1, 2, 3, 4, 5])
result5 = reverseBetween(head5, 3, 5)
print(list_to_array(result5))  # [1, 2, 5, 4, 3]
