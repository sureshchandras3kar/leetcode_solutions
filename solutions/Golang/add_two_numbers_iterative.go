package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/*
 * Add two numbers represented by linked lists in reverse order.
 *
 * Time Complexity: O(max(m, n)) where m and n are the lengths of the lists
 * Space Complexity: O(max(m, n)) for the output list
 */
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	dummy := &ListNode{Val: 0}
	current := dummy
	carry := 0

	for l1 != nil || l2 != nil || carry != 0 {
		val1 := 0
		if l1 != nil {
			val1 = l1.Val
		}
		val2 := 0
		if l2 != nil {
			val2 = l2.Val
		}

		total := val1 + val2 + carry
		carry = total / 10
		digit := total % 10

		current.Next = &ListNode{Val: digit}
		current = current.Next

		if l1 != nil {
			l1 = l1.Next
		}
		if l2 != nil {
			l2 = l2.Next
		}
	}

	return dummy.Next
}

// Helper function to create linked list from array
func createLinkedList(arr []int) *ListNode {
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

// Helper function to convert linked list to slice for printing
func linkedListToSlice(head *ListNode) []int {
	var result []int
	current := head
	for current != nil {
		result = append(result, current.Val)
		current = current.Next
	}
	return result
}

func main() {
	// Test case 1: [2,4,3] + [5,6,4] = [7,0,8] (342 + 465 = 807)
	l1 := createLinkedList([]int{2, 4, 3})
	l2 := createLinkedList([]int{5, 6, 4})
	result := addTwoNumbers(l1, l2)
	fmt.Println("Test 1:", linkedListToSlice(result))  // [7 0 8]

	// Test case 2: [0] + [0] = [0]
	l1 = createLinkedList([]int{0})
	l2 = createLinkedList([]int{0})
	result = addTwoNumbers(l1, l2)
	fmt.Println("Test 2:", linkedListToSlice(result))  // [0]

	// Test case 3: [9,9,9,9,9,9,9] + [9,9,9,9] = [8,9,9,9,0,0,0,1]
	l1 = createLinkedList([]int{9, 9, 9, 9, 9, 9, 9})
	l2 = createLinkedList([]int{9, 9, 9, 9})
	result = addTwoNumbers(l1, l2)
	fmt.Println("Test 3:", linkedListToSlice(result))  // [8 9 9 9 0 0 0 1]
}
