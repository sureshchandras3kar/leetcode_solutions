package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Reverse nodes in k-group recursively.
 *
 * Approach: Use recursion to handle each group independently.
 * - Check if there are k nodes remaining
 * - Reverse the first k nodes
 * - Recursively process the rest of the list
 * - Connect the reversed group to the result of the recursive call
 *
 * Time: O(n) - visit each node once
 * Space: O(n/k) - recursion depth is proportional to number of groups
 */
func reverseKGroup(head *ListNode, k int) *ListNode {
	// Find the k-th node
	kth := head
	for i := 0; i < k-1 && kth != nil; i++ {
		kth = kth.Next
	}

	// If no k-th node exists, cannot reverse, return original
	if kth == nil {
		return head
	}

	// Save the next group's head (after k-th node)
	nextGroupHead := kth.Next

	// Reverse from head to kth
	prev := nextGroupHead
	curr := head

	for curr != nextGroupHead {
		nextTemp := curr.Next
		curr.Next = prev
		prev = curr
		curr = nextTemp
	}

	// head is now the tail of the reversed group
	// prev is the new head (which was kth)
	// Recursively process the remaining list and connect
	head.Next = reverseKGroup(nextGroupHead, k)

	return prev // Return the new head (which was kth)
}

// Helper functions for testing
func createList(values []int) *ListNode {
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

func printList(head *ListNode) {
	fmt.Print("[")
	first := true
	for head != nil {
		if !first {
			fmt.Print(", ")
		}
		fmt.Print(head.Val)
		first = false
		head = head.Next
	}
	fmt.Println("]")
}

func main() {
	head1 := createList([]int{1, 2, 3, 4, 5})
	printList(reverseKGroup(head1, 2)) // [2, 1, 4, 3, 5]

	head2 := createList([]int{1, 2, 3, 4, 5})
	printList(reverseKGroup(head2, 3)) // [3, 2, 1, 4, 5]

	head3 := createList([]int{1})
	printList(reverseKGroup(head3, 1)) // [1]
}
