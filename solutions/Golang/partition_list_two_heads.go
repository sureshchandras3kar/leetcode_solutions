package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// Partition linked list into two groups: values < x and values >= x.
// Uses two separate list heads and merges them at the end.
//
// Time Complexity: O(n) where n is the number of nodes
// Space Complexity: O(1) excluding the output list
func partitionListTwoHeads(head *ListNode, x int) *ListNode {
	// Create dummy nodes for two lists
	smallerDummy := &ListNode{Val: 0}
	largerDummy := &ListNode{Val: 0}

	// Pointers to build the two lists
	smallerPtr := smallerDummy
	largerPtr := largerDummy

	// Partition nodes into two lists
	current := head
	for current != nil {
		if current.Val < x {
			smallerPtr.Next = current
			smallerPtr = smallerPtr.Next
		} else {
			largerPtr.Next = current
			largerPtr = largerPtr.Next
		}
		current = current.Next
	}

	// Close the larger list to avoid cycles
	largerPtr.Next = nil

	// Connect the two lists
	smallerPtr.Next = largerDummy.Next

	return smallerDummy.Next
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
	result := partitionListTwoHeads(head, 3)
	fmt.Println(linkedListToSlice(result))  // [1 2 2 4 3 5]

	head = createLinkedList([]int{2, 1})
	result = partitionListTwoHeads(head, 2)
	fmt.Println(linkedListToSlice(result))  // [1 2]

	head = createLinkedList([]int{5, 3, 1, 4, 2})
	result = partitionListTwoHeads(head, 3)
	fmt.Println(linkedListToSlice(result))  // [1 2 5 3 4]
}
