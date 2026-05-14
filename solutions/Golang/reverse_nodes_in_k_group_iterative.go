package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Reverse nodes in k-group iteratively.
 *
 * Approach: Reverse each group of k nodes in-place using iteration.
 * - Find the k-th node to determine the group boundary
 * - Reverse the group from current to k-th node
 * - Link the reversed group back to the previous group
 * - Move to the next group and repeat
 *
 * Time: O(n) - visit each node once
 * Space: O(1) - only pointers, no extra structures
 */
func reverseKGroup(head *ListNode, k int) *ListNode {
	getKthNode := func(node *ListNode, k int) *ListNode {
		for node != nil && k > 1 {
			node = node.Next
			k--
		}
		return node
	}

	// Dummy node to simplify logic
	dummy := &ListNode{Val: 0, Next: head}
	prevGroup := dummy

	for {
		// Check if there are at least k nodes remaining
		kthNode := getKthNode(prevGroup, k+1)
		if kthNode == nil {
			break
		}

		// Mark the start and end of the current group
		groupStart := prevGroup.Next
		groupEnd := kthNode

		// Save the next group's start
		nextGroupStart := groupEnd.Next

		// Reverse the current group
		prev := nextGroupStart
		curr := groupStart

		for curr != nextGroupStart {
			nextTemp := curr.Next
			curr.Next = prev
			prev = curr
			curr = nextTemp
		}

		// Link the previous group to the reversed current group
		prevGroup.Next = groupEnd
		prevGroup = groupStart
	}

	return dummy.Next
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
