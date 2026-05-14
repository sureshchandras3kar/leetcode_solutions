from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# Global variable to track the successor node
successor = None


def reverseBetween(head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
    """
    Reverse a portion of a linked list from position left to right using recursion.

    Time Complexity: O(n) - traverse to left, then reverse to right
    Space Complexity: O(n) - recursion call stack

    Strategy:
    1. If left == 1, call reverseN() to reverse the first 'right' nodes
    2. Otherwise, recursively process the next node with adjusted positions
    3. Use a global successor variable to track where to stop reversing
    """
    if left == 1:
        return reverseN(head, right)
    else:
        # Recursively process the rest of the list
        head.next = reverseBetween(head.next, left - 1, right - 1)
        return head


def reverseN(head: Optional[ListNode], n: int) -> Optional[ListNode]:
    """
    Reverse the first n nodes of a linked list.

    The global 'successor' variable tracks the node after the nth node,
    which becomes the tail of the reversed sublist.
    """
    global successor

    if n == 1:
        # Base case: mark successor as the node after the first node
        successor = head.next
        return head

    # Recursively reverse the rest
    new_head = reverseN(head.next, n - 1)

    # At this point, head.next points to the (n-1)th node
    # We want to reverse: head.next.next = head
    head.next.next = head

    # Connect head to successor (the node after position n)
    head.next = successor

    return new_head


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
