from typing import Optional, Tuple


class ListNode:
    def __init__(self, val: int = 0, next: Optional['ListNode'] = None):
        self.val = val
        self.next = next


def addTwoNumbers(l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
    """
    Add two numbers represented by linked lists in reverse order using recursion.

    Time Complexity: O(max(m, n)) where m and n are the lengths of the lists
    Space Complexity: O(max(m, n)) for the recursion call stack and output list
    """

    def helper(l1: Optional[ListNode], l2: Optional[ListNode], carry: int) -> Optional[ListNode]:
        # Base case: both lists are empty and no carry
        if not l1 and not l2 and carry == 0:
            return None

        val1 = l1.val if l1 else 0
        val2 = l2.val if l2 else 0

        total = val1 + val2 + carry
        digit = total % 10
        new_carry = total // 10

        # Move to next nodes
        next_l1 = l1.next if l1 else None
        next_l2 = l2.next if l2 else None

        # Recursively build the rest of the list
        next_node = helper(next_l1, next_l2, new_carry)

        # Create node with current digit
        return ListNode(digit, next_node)

    return helper(l1, l2, 0)


# Test cases
if __name__ == "__main__":
    # Helper function to create linked list from list
    def create_linked_list(arr):
        if not arr:
            return None
        head = ListNode(arr[0])
        current = head
        for val in arr[1:]:
            current.next = ListNode(val)
            current = current.next
        return head

    # Helper function to convert linked list to list for printing
    def linked_list_to_list(head):
        result = []
        current = head
        while current:
            result.append(current.val)
            current = current.next
        return result

    # Test case 1: [2,4,3] + [5,6,4] = [7,0,8] (342 + 465 = 807)
    l1 = create_linked_list([2, 4, 3])
    l2 = create_linked_list([5, 6, 4])
    result = addTwoNumbers(l1, l2)
    print(f"Test 1: {linked_list_to_list(result)}")  # [7, 0, 8]

    # Test case 2: [0] + [0] = [0]
    l1 = create_linked_list([0])
    l2 = create_linked_list([0])
    result = addTwoNumbers(l1, l2)
    print(f"Test 2: {linked_list_to_list(result)}")  # [0]

    # Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1] (9999999 + 9999 = 10009998)
    l1 = create_linked_list([9, 9, 9, 9, 9, 9, 9])
    l2 = create_linked_list([9, 9, 9, 9])
    result = addTwoNumbers(l1, l2)
    print(f"Test 3: {linked_list_to_list(result)}")  # [8, 9, 9, 9, 0, 0, 0, 1]
