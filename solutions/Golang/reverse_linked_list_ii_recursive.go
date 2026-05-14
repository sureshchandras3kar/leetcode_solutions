package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Reverse a portion of a linked list from position left to right using recursion.
 *
 * Time Complexity: O(n) - traverse to left, then reverse to right
 * Space Complexity: O(n) - recursion call stack
 *
 * Strategy:
 * 1. If left == 1, call reverseN() to reverse the first 'right' nodes
 * 2. Otherwise, recursively process the next node with adjusted positions
 */

// Global variable to track the successor node
var successor *ListNode

/**
 * Reverse the first n nodes of a linked list.
 *
 * The global 'successor' variable tracks the node after the nth node,
 * which becomes the tail of the reversed sublist.
 */
func reverseN(head *ListNode, n int) *ListNode {
	if n == 1 {
		// Base case: mark successor as the node after the first node
		successor = head.Next
		return head
	}

	// Recursively reverse the rest
	newHead := reverseN(head.Next, n-1)

	// At this point, head.Next points to the (n-1)th node
	// We want to reverse: head.Next.Next = head
	head.Next.Next = head

	// Connect head to successor (the node after position n)
	head.Next = successor

	return newHead
}

/**
 * Reverse a portion of a linked list from position left to right.
 */
func reverseBetween(head *ListNode, left int, right int) *ListNode {
	if left == 1 {
		return reverseN(head, right)
	} else {
		// Recursively process the rest of the list
		head.Next = reverseBetween(head.Next, left-1, right-1)
		return head
	}
}

// Helper function to create a linked list from a slice
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

// Helper function to convert linked list to slice for comparison
func listToSlice(head *ListNode) []int {
	var result []int
	for head != nil {
		result = append(result, head.Val)
		head = head.Next
	}
	return result
}

// Test cases
func main() {
	// Test 1: Reverse middle portion
	head1 := createList([]int{1, 2, 3, 4, 5})
	successor = nil
	result1 := reverseBetween(head1, 2, 4)
	fmt.Println("Test 1:", listToSlice(result1)) // [1 4 3 2 5]

	// Test 2: Single node (no reversal)
	head2 := createList([]int{5})
	successor = nil
	result2 := reverseBetween(head2, 1, 1)
	fmt.Println("Test 2:", listToSlice(result2)) // [5]

	// Test 3: Reverse entire list
	head3 := createList([]int{1, 2})
	successor = nil
	result3 := reverseBetween(head3, 1, 2)
	fmt.Println("Test 3:", listToSlice(result3)) // [2 1]

	// Test 4: Reverse from start
	head4 := createList([]int{1, 2, 3, 4, 5})
	successor = nil
	result4 := reverseBetween(head4, 1, 3)
	fmt.Println("Test 4:", listToSlice(result4)) // [3 2 1 4 5]

	// Test 5: Reverse at end
	head5 := createList([]int{1, 2, 3, 4, 5})
	successor = nil
	result5 := reverseBetween(head5, 3, 5)
	fmt.Println("Test 5:", listToSlice(result5)) // [1 2 5 4 3]
}
