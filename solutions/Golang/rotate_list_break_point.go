package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// RotateListBreakPoint rotates a linked list to the right by k positions using the break point approach.
//
// Approach:
// 1. Calculate the effective rotation: k = k % length
// 2. Find the break point: the node at position (length - k - 1)
// 3. Perform rotation by breaking the list at the break point
//
// Time: O(n) - single pass to find length, single pass to find break point
// Space: O(1) - only using pointers
func RotateListBreakPoint(head *ListNode, k int) *ListNode {
	if head == nil || head.Next == nil || k == 0 {
		return head
	}

	// Step 1: Find the length of the list
	length := 0
	current := head
	for current != nil {
		length++
		current = current.Next
	}

	// Step 2: Normalize k (handle cases where k > length)
	k = k % length
	if k == 0 {
		return head
	}

	// Step 3: Find the break point (node before rotation point)
	// We need to find the (length - k - 1)th node
	current = head
	for i := 0; i < length-k-1; i++ {
		current = current.Next
	}

	// Step 4: Perform rotation
	// The new head is current.Next
	newHead := current.Next
	current.Next = nil

	// Find the tail of the new list and connect it back to the old head
	tail := newHead
	for tail.Next != nil {
		tail = tail.Next
	}
	tail.Next = head

	return newHead
}

// Helper function to create a linked list from a slice
func CreateLinkedList(values []int) *ListNode {
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
func LinkedListToSlice(head *ListNode) []int {
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
	// Test 1: [1, 2, 3, 4, 5], k=2
	head1 := CreateLinkedList([]int{1, 2, 3, 4, 5})
	result1 := RotateListBreakPoint(head1, 2)
	fmt.Println(LinkedListToSlice(result1))  // [4, 5, 1, 2, 3]

	// Test 2: [0, 1, 2], k=4
	head2 := CreateLinkedList([]int{0, 1, 2})
	result2 := RotateListBreakPoint(head2, 4)
	fmt.Println(LinkedListToSlice(result2))  // [2, 0, 1]

	// Test 3: [1], k=1
	head3 := CreateLinkedList([]int{1})
	result3 := RotateListBreakPoint(head3, 1)
	fmt.Println(LinkedListToSlice(result3))  // [1]

	// Test 4: [1, 2], k=2
	head4 := CreateLinkedList([]int{1, 2})
	result4 := RotateListBreakPoint(head4, 2)
	fmt.Println(LinkedListToSlice(result4))  // [1, 2]

	// Test 5: [1, 2, 3, 4, 5], k=0
	head5 := CreateLinkedList([]int{1, 2, 3, 4, 5})
	result5 := RotateListBreakPoint(head5, 0)
	fmt.Println(LinkedListToSlice(result5))  // [1, 2, 3, 4, 5]
}
