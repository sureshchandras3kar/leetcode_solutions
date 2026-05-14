package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

/**
 * Merge two sorted linked lists recursively.
 *
 * Time Complexity: O(m + n) where m and n are the lengths of list1 and list2
 * Space Complexity: O(m + n) due to the recursion call stack
 */
func mergeTwoSortedListsRecursive(list1 *ListNode, list2 *ListNode) *ListNode {
	// Base cases: if one list is empty, return the other
	if list1 == nil {
		return list2
	}
	if list2 == nil {
		return list1
	}

	// Compare the values and recursively merge
	if list1.Val <= list2.Val {
		list1.Next = mergeTwoSortedListsRecursive(list1.Next, list2)
		return list1
	} else {
		list2.Next = mergeTwoSortedListsRecursive(list1, list2.Next)
		return list2
	}
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
	result := mergeTwoSortedListsRecursive(list1, list2)
	fmt.Println(linkedListToSlice(result))  // [1 1 2 3 4 4]

	list1 = createLinkedList([]int{})
	list2 = createLinkedList([]int{})
	result = mergeTwoSortedListsRecursive(list1, list2)
	fmt.Println(linkedListToSlice(result))  // []

	list1 = createLinkedList([]int{})
	list2 = createLinkedList([]int{0})
	result = mergeTwoSortedListsRecursive(list1, list2)
	fmt.Println(linkedListToSlice(result))  // [0]
}
