package main

import "fmt"

type ListNode struct {
    Val  int
    Next *ListNode
}

/*
Remove the nth node from the end of a linked list using two passes.

Approach:
1. First pass: count the total number of nodes
2. Second pass: find and skip the nth node from the end

Time: O(L) + O(L-n) = O(L)
Space: O(1)
*/
func removeNthNodeTwoPass(head *ListNode, n int) *ListNode {
    // First pass: count the total nodes
    length := 0
    current := head
    for current != nil {
        length++
        current = current.Next
    }

    // Edge case: removing the head node
    if length == n {
        return head.Next
    }

    // Second pass: find the node before the one to remove
    current = head
    for i := 0; i < length-n-1; i++ {
        current = current.Next
    }

    // Remove the nth node
    current.Next = current.Next.Next
    return head
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
    result1 := removeNthNodeTwoPass(head1, 2)
    fmt.Println(linkedListToSlice(result1))  // [1 2 3 5]

    // Test 2: [1], n=1
    head2 := createLinkedList([]int{1})
    result2 := removeNthNodeTwoPass(head2, 1)
    fmt.Println(linkedListToSlice(result2))  // []

    // Test 3: [1, 2], n=2
    head3 := createLinkedList([]int{1, 2})
    result3 := removeNthNodeTwoPass(head3, 2)
    fmt.Println(linkedListToSlice(result3))  // [2]

    // Test 4: [1, 2], n=1
    head4 := createLinkedList([]int{1, 2})
    result4 := removeNthNodeTwoPass(head4, 1)
    fmt.Println(linkedListToSlice(result4))  // [1]
}
