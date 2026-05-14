package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func sortList(head *ListNode) *ListNode {
	if head == nil {
		return head
	}

	dummy := &ListNode{Next: head}

	length := 0
	curr := head
	for curr != nil {
		length++
		curr = curr.Next
	}

	for size := 1; size < length; size *= 2 {
		prev := dummy
		curr := dummy.Next

		for curr != nil {
			l1 := curr
			l1Tail := l1
			l1Len := 0
			for l1Len < size && l1Tail != nil {
				l1Tail = l1Tail.Next
				l1Len++
			}

			l2 := l1Tail
			l2Len := 0
			for l2Len < size && l2 != nil {
				l2 = l2.Next
				l2Len++
			}

			l1Tail = l1
			for i := 1; i < l1Len; i++ {
				if l1Tail != nil {
					l1Tail = l1Tail.Next
				}
			}
			if l1Tail != nil {
				l1Tail.Next = nil
			}

			prev.Next = merge(l1, l2)
			for prev.Next != nil {
				prev = prev.Next
			}
			curr = l2
		}
	}

	return dummy.Next
}

func merge(l1, l2 *ListNode) *ListNode {
	dummy := &ListNode{}
	curr := dummy
	for l1 != nil && l2 != nil {
		if l1.Val <= l2.Val {
			curr.Next = l1
			l1 = l1.Next
		} else {
			curr.Next = l2
			l2 = l2.Next
		}
		curr = curr.Next
	}
	if l1 != nil {
		curr.Next = l1
	} else {
		curr.Next = l2
	}
	return dummy.Next
}

func main() {
	head := &ListNode{Val: 4}
	head.Next = &ListNode{Val: 2}
	head.Next.Next = &ListNode{Val: 1}
	head.Next.Next.Next = &ListNode{Val: 3}

	result := sortList(head)
	for result != nil {
		fmt.Print(result.Val, " ")
		result = result.Next
	}
	fmt.Println()
}
