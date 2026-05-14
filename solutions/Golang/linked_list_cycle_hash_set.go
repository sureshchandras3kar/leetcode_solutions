package main

import "fmt"

type ListNodeHash struct {
	Val  int
	Next *ListNodeHash
}

func hasCycleHash(head *ListNodeHash) bool {
	seen := make(map[*ListNodeHash]bool)
	current := head

	for current != nil {
		if seen[current] {
			return true
		}
		seen[current] = true
		current = current.Next
	}

	return false
}

func mainHash() {
	// Test case 1: Cycle exists
	node1 := &ListNodeHash{Val: 3}
	node2 := &ListNodeHash{Val: 2}
	node3 := &ListNodeHash{Val: 0}
	node4 := &ListNodeHash{Val: -4}
	node1.Next = node2
	node2.Next = node3
	node3.Next = node4
	node4.Next = node2  // Cycle

	fmt.Println("Cycle exists:", hasCycleHash(node1))  // true

	// Test case 2: No cycle
	node5 := &ListNodeHash{Val: 1}
	node6 := &ListNodeHash{Val: 2}
	node5.Next = node6
	fmt.Println("Cycle exists:", hasCycleHash(node5))  // false

	// Test case 3: Single node
	node7 := &ListNodeHash{Val: 1}
	fmt.Println("Cycle exists:", hasCycleHash(node7))  // false
}
