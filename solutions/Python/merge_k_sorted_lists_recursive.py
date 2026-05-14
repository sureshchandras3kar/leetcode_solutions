from typing import Optional, List
import heapq

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def mergeKLists(lists: List[Optional[ListNode]]) -> Optional[ListNode]:
    if not lists:
        return None
    
    def merge(l1, l2):
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
    
    def partition(lists, left, right):
        if left == right:
            return lists[left]
        if left > right:
            return None
        
        mid = (left + right) // 2
        l1 = partition(lists, left, mid)
        l2 = partition(lists, mid + 1, right)
        return merge(l1, l2)
    
    return partition(lists, 0, len(lists) - 1)

# Example
l1 = ListNode(1, ListNode(4, ListNode(5)))
l2 = ListNode(1, ListNode(3, ListNode(4)))
l3 = ListNode(2, ListNode(6))
result = mergeKLists([l1, l2, l3])
while result:
    print(result.val, end=" ")
    result = result.next
