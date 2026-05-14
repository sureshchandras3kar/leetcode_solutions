package main

import "fmt"

type ListNodeFloyd struct {
	Val  int
	Next *ListNodeFloyd
}

func hasCycleFloyd(head *ListNodeFloyd) bool {
	if head == nil || head.Next == nil {
		return false
	}

	slow := head
	fast := head

	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next

		if slow == fast {
			return true
		}
	}

	return false
}

func mainFloyd() {
	// Test case 1: Cycle exists
	node1 := &ListNodeFloyd{Val: 3}
	node2 := &ListNodeFloyd{Val: 2}
	node3 := &ListNodeFloyd{Val: 0}
	node4 := &ListNodeFloyd{Val: -4}
	node1.Next = node2
	node2.Next = node3
	node3.Next = node4
	node4.Next = node2  // Cycle

	fmt.Println("Cycle exists:", hasCycleFloyd(node1))  // true

	// Test case 2: No cycle
	node5 := &ListNodeFloyd{Val: 1}
	node6 := &ListNodeFloyd{Val: 2}
	node5.Next = node6
	fmt.Println("Cycle exists:", hasCycleFloyd(node5))  // false

	// Test case 3: Single node
	node7 := &ListNodeFloyd{Val: 1}
	fmt.Println("Cycle exists:", hasCycleFloyd(node7))  // false
}
