package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Remove all nodes with duplicate values from sorted linked list in single pass.
 *
 * Approach: Use a dummy node and prev pointer. When we find duplicates, skip all
 * nodes with that value by advancing current and updating prev.Next.
 * Time: O(n) — single pass through the list
 * Space: O(1) — only pointer variables
 */
func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	// Create dummy node to handle edge case where head is duplicate
	dummy := &ListNode{Val: 0}
	dummy.Next = head
	prev := dummy
	current := head

	for current != nil {
		// Check if current node is the start of a duplicate group
		if current.Next != nil && current.Val == current.Next.Val {
			// Skip all nodes with the same value
			value := current.Val
			for current != nil && current.Val == value {
				current = current.Next
			}
			// Link prev to the first non-duplicate node
			prev.Next = current
		} else {
			// Current node is unique, keep it
			prev = current
			current = current.Next
		}
	}

	return dummy.Next
}

// Helper function to create linked list from array
func createList(arr []int) *ListNode {
	if len(arr) == 0 {
		return nil
	}
	head := &ListNode{Val: arr[0]}
	current := head
	for i := 1; i < len(arr); i++ {
		current.Next = &ListNode{Val: arr[i]}
		current = current.Next
	}
	return head
}

// Helper function to print linked list
func printList(head *ListNode) {
	for head != nil {
		fmt.Printf("%d -> ", head.Val)
		head = head.Next
	}
	fmt.Println("null")
}

func main() {
	// Test case 1: [1,2,3,3,4,4,5]
	head1 := createList([]int{1, 2, 3, 3, 4, 4, 5})
	fmt.Print("Test 1: ")
	printList(deleteDuplicates(head1))  // 1 -> 2 -> 5 -> null

	// Test case 2: [1,1,1,2,3]
	head2 := createList([]int{1, 1, 1, 2, 3})
	fmt.Print("Test 2: ")
	printList(deleteDuplicates(head2))  // 2 -> 3 -> null

	// Test case 3: [1,1]
	head3 := createList([]int{1, 1})
	fmt.Print("Test 3: ")
	printList(deleteDuplicates(head3))  // null
}
