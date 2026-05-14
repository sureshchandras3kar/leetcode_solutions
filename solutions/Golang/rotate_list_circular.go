package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// RotateListCircular rotates a linked list to the right by k positions using the circular approach.
//
// Approach:
// 1. Calculate the effective rotation: k = k % length
// 2. Create a circular list by connecting the tail to the head
// 3. Find the new head position: walk (length - k) steps from the original head
// 4. Break the circle at the appropriate point
//
// Time: O(n) - single pass to find length and establish circle, walk (length - k) steps
// Space: O(1) - only using pointers
func RotateListCircular(head *ListNode, k int) *ListNode {
	if head == nil || head.Next == nil || k == 0 {
		return head
	}

	// Step 1: Find the length of the list and the tail
	length := 0
	tail := head
	for tail.Next != nil {
		length++
		tail = tail.Next
	}
	length++  // Add 1 for the tail node itself

	// Step 2: Normalize k
	k = k % length
	if k == 0 {
		return head
	}

	// Step 3: Create a circular list
	tail.Next = head

	// Step 4: Find the new head position
	// After rotation by k, the new head is at position (length - k)
	// We need to walk (length - k) steps from the original head
	stepsToWalk := length - k
	current := head
	for i := 0; i < stepsToWalk-1; i++ {
		current = current.Next
	}

	// Step 5: Break the circle
	newHead := current.Next
	current.Next = nil

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
	result1 := RotateListCircular(head1, 2)
	fmt.Println(LinkedListToSlice(result1))  // [4, 5, 1, 2, 3]

	// Test 2: [0, 1, 2], k=4
	head2 := CreateLinkedList([]int{0, 1, 2})
	result2 := RotateListCircular(head2, 4)
	fmt.Println(LinkedListToSlice(result2))  // [2, 0, 1]

	// Test 3: [1], k=1
	head3 := CreateLinkedList([]int{1})
	result3 := RotateListCircular(head3, 1)
	fmt.Println(LinkedListToSlice(result3))  // [1]

	// Test 4: [1, 2], k=2
	head4 := CreateLinkedList([]int{1, 2})
	result4 := RotateListCircular(head4, 2)
	fmt.Println(LinkedListToSlice(result4))  // [1, 2]

	// Test 5: [1, 2, 3, 4, 5], k=0
	head5 := CreateLinkedList([]int{1, 2, 3, 4, 5})
	result5 := RotateListCircular(head5, 0)
	fmt.Println(LinkedListToSlice(result5))  // [1, 2, 3, 4, 5]
}
