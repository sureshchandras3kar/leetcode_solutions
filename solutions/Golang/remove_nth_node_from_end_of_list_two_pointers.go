package main

import "fmt"

type ListNode struct {
    Val  int
    Next *ListNode
}

/*
Remove the nth node from the end of a linked list using two pointers in one pass.

Approach:
1. Create a dummy node pointing to head (handles edge case of removing head)
2. Use fast and slow pointers with a gap of n nodes between them
3. Move both pointers until fast reaches the end
4. Skip the target node by adjusting the slow pointer

Time: O(L) single pass
Space: O(1)
*/
func removeNthNodeTwoPointers(head *ListNode, n int) *ListNode {
    // Create a dummy node to handle edge case of removing the head
    dummy := &ListNode{Val: 0}
    dummy.Next = head
    slow := dummy
    fast := dummy

    // Move fast pointer n+1 steps ahead
    for i := 0; i <= n; i++ {
        if fast == nil {
            return head
        }
        fast = fast.Next
    }

    // Move both pointers until fast reaches the end
    for fast != nil {
        slow = slow.Next
        fast = fast.Next
    }

    // Remove the nth node
    slow.Next = slow.Next.Next
    return dummy.Next
}

// Helper function to create a linked list from a slice
func createLinkedList(values []int) *ListNode {
    if len(values) == 0 {
        return nil
    }
    head := &ListNode{Val: values[0]}
    current := head
    for i := 1; i < len(values); i++ {
        current.Next = &ListNode{Val: values[i]}
        current = current.Next
    }
    return head
}

// Helper function to convert linked list to slice for easy printing
func linkedListToSlice(head *ListNode) []int {
    var result []int
    current := head
    for current != nil {
        result = append(result, current.Val)
        current = current.Next
    }
    return result
}

// Test cases
func main() {
    // Test 1: [1, 2, 3, 4, 5], n=2
    head1 := createLinkedList([]int{1, 2, 3, 4, 5})
    result1 := removeNthNodeTwoPointers(head1, 2)
    fmt.Println(linkedListToSlice(result1))  // [1 2 3 5]

    // Test 2: [1], n=1
    head2 := createLinkedList([]int{1})
    result2 := removeNthNodeTwoPointers(head2, 1)
    fmt.Println(linkedListToSlice(result2))  // []

    // Test 3: [1, 2], n=2
    head3 := createLinkedList([]int{1, 2})
    result3 := removeNthNodeTwoPointers(head3, 2)
    fmt.Println(linkedListToSlice(result3))  // [2]

    // Test 4: [1, 2], n=1
    head4 := createLinkedList([]int{1, 2})
    result4 := removeNthNodeTwoPointers(head4, 1)
    fmt.Println(linkedListToSlice(result4))  // [1]
}
