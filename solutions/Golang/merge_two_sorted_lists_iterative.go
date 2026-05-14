package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Merge two sorted linked lists iteratively.
 *
 * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
 * Space Complexity: O(1) excluding the output list
 */
func mergeTwoSortedListsIterative(list1 *ListNode, list2 *ListNode) *ListNode {
	// Create a dummy node to simplify the code
	dummy := &ListNode{Val: 0}
	current := dummy

	// Traverse both lists and append the smaller node
	for list1 != nil && list2 != nil {
		if list1.Val <= list2.Val {
			current.Next = list1
			list1 = list1.Next
		} else {
			current.Next = list2
			list2 = list2.Next
		}
		current = current.Next
	}

	// Append the remaining nodes
	if list1 != nil {
		current.Next = list1
	} else {
		current.Next = list2
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

// Helper function to convert linked list to slice for testing
func linkedListToSlice(node *ListNode) []int {
	var result []int
	for node != nil {
		result = append(result, node.Val)
		node = node.Next
	}
	return result
}

func main() {
	list1 := createLinkedList([]int{1, 2, 4})
	list2 := createLinkedList([]int{1, 3, 4})
	result := mergeTwoSortedListsIterative(list1, list2)
	fmt.Println(linkedListToSlice(result))  // [1 1 2 3 4 4]

	list1 = createLinkedList([]int{})
	list2 = createLinkedList([]int{})
	result = mergeTwoSortedListsIterative(list1, list2)
	fmt.Println(linkedListToSlice(result))  // []

	list1 = createLinkedList([]int{})
	list2 = createLinkedList([]int{0})
	result = mergeTwoSortedListsIterative(list1, list2)
	fmt.Println(linkedListToSlice(result))  // [0]
}
