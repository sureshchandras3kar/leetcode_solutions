from typing import Optional


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def sortList(head: Optional[ListNode]) -> Optional[ListNode]:
    if not head:
        return head

    length = 0
    curr = head
    while curr:
        length += 1
        curr = curr.next

    dummy = ListNode(0, head)
    size = 1

    while size < length:
        curr = dummy.next
        tail = dummy

        while curr:
            l1 = curr
            l1_end = l1
            l1_len = 0

            while l1_len < size and l1_end:
                l1_end = l1_end.next
                l1_len += 1

            l2 = l1_end
            l2_len = 0

            while l2_len < size and l2:
                l2 = l2.next
                l2_len += 1

            l1_end = l1
            for _ in range(l1_len - 1):
                if l1_end:
                    l1_end = l1_end.next

            if l1_end:
                l1_end.next = None

            tail.next = merge(l1, l2)

            while tail.next:
                tail = tail.next

            curr = l2

        size *= 2

    return dummy.next


def merge(l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
    dummy = ListNode(0)
    curr = dummy
    while l1 and l2:
        if l1.val <= l2.val:
            curr.next = l1
            l1 = l1.next
        else:
            curr.next = l2
            l2 = l2.next
        curr = curr.next
    curr.next = l1 if l1 else l2
    return dummy.next


# Example usage
head = ListNode(4, ListNode(2, ListNode(1, ListNode(3))))
result = sortList(head)
while result:
    print(result.val, end=" ")
    result = result.next
print()
