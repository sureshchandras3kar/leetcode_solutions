package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Remove all nodes with duplicate values from sorted linked list using hash set.
 *
 * Approach: Count frequency of each value, then rebuild list with unique values only.
 * Time: O(n) — two passes through the list
 * Space: O(n) — hash map stores frequencies
 */
func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	// Count frequencies
	freq := make(map[int]int)
	current := head
	for current != nil {
		freq[current.Val]++
		current = current.Next
	}

	// Build result list with unique values only
	dummy := &ListNode{Val: 0}
	dummy.Next = head
	prev := dummy
	current = head

	for current != nil {
		if freq[current.Val] == 1 {
			// Keep unique node
			prev = current
			current = current.Next
		} else {
			// Skip duplicate node
			prev.Next = current.Next
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
