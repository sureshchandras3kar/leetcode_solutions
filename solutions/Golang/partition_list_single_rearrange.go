package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// Partition linked list into two groups: values < x and values >= x.
// Rearranges nodes in a single pass without creating separate lists.
//
// Time Complexity: O(n) where n is the number of nodes
// Space Complexity: O(1) excluding the output list
func partitionListSingleRearrange(head *ListNode, x int) *ListNode {
	// Create a dummy node to simplify edge cases
	dummy := &ListNode{Val: 0}
	dummy.Next = head

	// Find the node just before the partition point
	prev := dummy
	current := head

	for current != nil && current.Val < x {
		prev = current
		current = current.Next
	}

	// Now prev is the last node with value < x (or dummy if all nodes >= x)
	// current is the first node with value >= x (or nil if all nodes < x)

	// If all nodes are less than x or list is empty, return as is
	if current == nil {
		return dummy.Next
	}

	// Now we need to insert nodes with value < x before the partition point
	partitionPrev := prev  // Last node of < x group
	run := current         // Start from the partition point

	for run != nil {
		if run.Val < x {
			// Remove run from its position
			current.Next = run.Next

			// Insert run before the partition point
			run.Next = partitionPrev.Next
			partitionPrev.Next = run

			// Move partitionPrev forward
			partitionPrev = run

			// Current stays the same to check the next node
			run = current.Next
		} else {
			// Move forward
			current = run
			run = run.Next
		}
	}

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

// Helper function to convert linked list to slice for easy testing
func linkedListToSlice(node *ListNode) []int {
	result := []int{}
	for node != nil {
		result = append(result, node.Val)
		node = node.Next
	}
	return result
}

// Test cases
func main() {
	head := createLinkedList([]int{1, 4, 3, 2, 5, 2})
	result := partitionListSingleRearrange(head, 3)
	fmt.Println(linkedListToSlice(result))  // [1 2 2 4 3 5]

	head = createLinkedList([]int{2, 1})
	result = partitionListSingleRearrange(head, 2)
	fmt.Println(linkedListToSlice(result))  // [1 2]

	head = createLinkedList([]int{5, 3, 1, 4, 2})
	result = partitionListSingleRearrange(head, 3)
	fmt.Println(linkedListToSlice(result))  // [1 2 5 3 4]
}
