package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Reverse a portion of a linked list from position left to right (1-indexed).
 *
 * Time Complexity: O(n) - single pass through the list
 * Space Complexity: O(1) - only pointer variables
 *
 * Strategy:
 * 1. Create a dummy node to handle edge case of reversing from head
 * 2. Advance prev pointer to position left-1
 * 3. Perform (right - left) reversals by moving nodes to the front of the sublist
 * 4. Return dummy.Next
 */
func reverseBetween(head *ListNode, left int, right int) *ListNode {
	if left == right {
		return head
	}

	// Create dummy node pointing to head
	dummy := &ListNode{Val: 0, Next: head}
	prev := dummy

	// Advance prev to position left-1
	for i := 0; i < left-1; i++ {
		prev = prev.Next
	}

	// curr is the first node to reverse (at position left)
	curr := prev.Next

	// Perform (right - left) reversals
	for i := 0; i < right-left; i++ {
		// nextTemp is the node we want to move to the front
		nextTemp := curr.Next
		// Bypass nextTemp by connecting curr to nextTemp.Next
		curr.Next = nextTemp.Next
		// Move nextTemp to the front of the sublist
		nextTemp.Next = prev.Next
		prev.Next = nextTemp
	}

	return dummy.Next
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
	result1 := reverseBetween(head1, 2, 4)
	fmt.Println("Test 1:", listToSlice(result1)) // [1 4 3 2 5]

	// Test 2: Single node (no reversal)
	head2 := createList([]int{5})
	result2 := reverseBetween(head2, 1, 1)
	fmt.Println("Test 2:", listToSlice(result2)) // [5]

	// Test 3: Reverse entire list
	head3 := createList([]int{1, 2})
	result3 := reverseBetween(head3, 1, 2)
	fmt.Println("Test 3:", listToSlice(result3)) // [2 1]

	// Test 4: Reverse from start
	head4 := createList([]int{1, 2, 3, 4, 5})
	result4 := reverseBetween(head4, 1, 3)
	fmt.Println("Test 4:", listToSlice(result4)) // [3 2 1 4 5]

	// Test 5: Reverse at end
	head5 := createList([]int{1, 2, 3, 4, 5})
	result5 := reverseBetween(head5, 3, 5)
	fmt.Println("Test 5:", listToSlice(result5)) // [1 2 5 4 3]
}
