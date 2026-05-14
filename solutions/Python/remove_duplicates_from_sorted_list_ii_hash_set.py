from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def deleteDuplicates(head: Optional[ListNode]) -> Optional[ListNode]:
    """
    Remove all nodes with duplicate values from sorted linked list using hash set.

    Approach: Count frequency of each value, then rebuild the list with unique values only.
    Time: O(n) — two passes through the list
    Space: O(n) — hash map stores frequencies
    """
    if not head:
        return None

    # Count frequencies
    freq = {}
    current = head
    while current:
        freq[current.val] = freq.get(current.val, 0) + 1
        current = current.next

    # Build result list with unique values only
    dummy = ListNode(0)
    dummy.next = head
    prev = dummy
    current = head

    while current:
        if freq[current.val] == 1:
            # Keep unique node
            prev = current
            current = current.next
        else:
            # Skip duplicate node
            prev.next = current.next
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
